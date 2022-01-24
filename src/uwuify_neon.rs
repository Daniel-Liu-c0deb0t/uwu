#![cfg(any(target_arch = "arm", target_arch = "aarch64"))]

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;
#[cfg(all(target_arch = "arm", target_feature = "neon"))]
use std::arch::asm;

use std::{ptr, str};

use crate::bitap_neon::{movemask, Bitap8x16};
use crate::rng::XorShift32;

use super::{pad_zeros, round_up16, LUT, LUT_LEN};

/// uwuify a string slice
///
/// requires the neon feature
///
/// this is probably fine for one-off use, but not very efficient if called multiple times.
/// use `uwuify_neon` to reduce memory allocations
///
/// # example:
/// ```
/// use uwuifier::uwuify_str;
/// assert_eq!(uwuify_str("hello world"), "hewwo wowwd");
/// ```
pub fn uwuify_str_neon(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut temp1 = vec![0u8; round_up16(bytes.len()) * 16];
    let mut temp2 = vec![0u8; round_up16(bytes.len()) * 16];
    unsafe { str::from_utf8_unchecked(uwuify_neon(bytes, &mut temp1, &mut temp2)).to_owned() }
}

#[inline(always)]
unsafe fn shift_left(a: uint8x16_t, imm: usize) -> uint8x16_t {
    let tmp = [vdupq_n_u8(0), a];
    vld1q_u8((tmp.as_ptr() as *mut u8).add(16 - imm))
}

#[inline(always)]
unsafe fn shift_right(a: uint8x16_t, imm: usize) -> uint8x16_t {
    let tmp = [a, vdupq_n_u8(0)];
    vld1q_u8((tmp.as_ptr() as *mut u8).add(imm))
}

#[inline(always)]
unsafe fn shuffle(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    let idx_mask = vandq_u8(b, vdupq_n_u8(0x8F));

    #[cfg(target_arch = "aarch64")]
    return vqtbl1q_u8(a, idx_mask);
    #[cfg(all(target_arch = "arm", target_feature = "neon"))]
    {
        let ret;
        asm!(
                "vtbl.8  {ret:e}, {{ {tbl:e}, {tbl:f} }}, {idx:e}",
                "vtbl.8  {ret:f}, {{ {tbl:e}, {tbl:f} }}, {idx:f}",
                ret = out(qreg) ret,
                tbl = in(qreg) a,
                idx = in(qreg) idx_mask,
                options(nostack)
        );
        return ret;
    }
}

#[inline(always)]
unsafe fn blend(a: uint8x16_t, b: uint8x16_t, mask: uint8x16_t) -> uint8x16_t {
    let mask = vshrq_n_u8::<7>(mask);
    vbslq_u8(mask, b, a)
}

/// uwuify some bytes
///
/// requires the neon feature
///
/// `temp_bytes1` and `temp_bytes2` must be buffers of size `round_up16(bytes.len()) * 16`,
/// because this is the worst-case size of the output. yes, it is annoying to allocate by
/// hand, but simd :)
///
/// the returned slice is the uwu'd result. when working with utf-8 strings, just pass in
/// the string as raw bytes and convert the output slice back to a string afterwards.
/// there's also the `uwuify_str_neon` function that is suitable for one-off use with a string slice
///
/// # example:
/// ```
/// use uwuifier::{uwuify, round_up16};
/// let s = "hello world";
/// let b = s.as_bytes();
/// let mut temp1 = vec![0u8; round_up16(b.len()) * 16];
/// let mut temp2 = vec![0u8; round_up16(b.len()) * 16];
/// let res = uwuify(b, &mut temp1, &mut temp2);
/// assert_eq!(std::str::from_utf8(res).unwrap(), "hewwo wowwd");
/// ```
pub fn uwuify_neon<'a>(
    bytes: &[u8],
    temp_bytes1: &'a mut [u8],
    temp_bytes2: &'a mut [u8],
) -> &'a [u8] {
    assert!(temp_bytes1.len() >= round_up16(bytes.len()) * 16);
    assert!(temp_bytes2.len() >= round_up16(bytes.len()) * 16);

    // only the highest quality seed will do
    let mut rng = XorShift32::new(b"uwu!");

    let mut len = bytes.len();

    unsafe {
        // bitap_sse will not read past len, unlike the other passes
        len = bitap_neon(bytes, len, temp_bytes1);
        pad_zeros(temp_bytes1, len);
        len = nya_ify_neon(temp_bytes1, len, temp_bytes2);
        pad_zeros(temp_bytes2, len);
        len = replace_and_stutter_neon(&mut rng, temp_bytes2, len, temp_bytes1);
        pad_zeros(temp_bytes1, len);
        len = emoji_neon(&mut rng, temp_bytes1, len, temp_bytes2);
        &temp_bytes2[..len]
    }
}

#[target_feature(enable = "neon")]
unsafe fn bitap_neon(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let mut out_ptr = out_bytes.as_mut_ptr();
    let mut bitap = Bitap8x16::new();
    let iter_len = len;

    for i in 0..iter_len {
        let c = *in_bytes.get_unchecked(i);
        ptr::write(out_ptr, c);
        out_ptr = out_ptr.add(1);

        if let Some(m) = bitap.next(c) {
            let replace = vld1q_s32(m.replace_ptr as *const i32);
            vst1q_s32(out_ptr.sub(m.match_len) as *mut i32, replace);
            out_ptr = out_ptr.add(m.replace_len).sub(m.match_len);
            len = len + m.replace_len - m.match_len;
            bitap.reset();
        }
    }

    len
}

#[target_feature(enable = "neon")]
unsafe fn emoji_neon(
    rng: &mut XorShift32,
    in_bytes: &[u8],
    mut len: usize,
    out_bytes: &mut [u8],
) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let splat_period = vdupq_n_u8(b'.');
    let splat_comma = vdupq_n_u8(b',');
    let splat_exclamation = vdupq_n_u8(b'!');
    let splat_space = vdupq_n_u8(b' ');
    let splat_tab = vdupq_n_u8(b'\t');
    let splat_newline = vdupq_n_u8(b'\n');
    let indexes = vld1q_u8(&[0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] as _);

    let lut_bits = LUT.len().trailing_zeros() as u32;

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        let vec = vld1q_u8(in_ptr.add(i));
        let mut punctuation_mask = vorrq_u8(
            vceqq_u8(vec, splat_comma),
            vorrq_u8(
                vceqq_u8(vec, splat_period),
                vceqq_u8(vec, splat_exclamation),
            ),
        );
        // multiple punctuation in a row means no emoji
        let mut multiple_mask = vandq_u8(punctuation_mask, shift_left(punctuation_mask, 1));
        multiple_mask = vorrq_u8(multiple_mask, shift_right(multiple_mask, 1));
        // punctuation must be followed by a space or else no emoji
        let space_mask = vorrq_u8(
            vceqq_u8(vec, splat_space),
            vorrq_u8(vceqq_u8(vec, splat_tab), vceqq_u8(vec, splat_newline)),
        );
        punctuation_mask = vbicq_u8(
            vandq_u8(punctuation_mask, shift_right(space_mask, 1)),
            multiple_mask,
        );
        let insert_mask = movemask(punctuation_mask) as u32;

        vst1q_u8(out_ptr, vec);

        // be lazy and only allow one emoji per vector
        if insert_mask != 0 {
            let insert_idx = insert_mask.trailing_zeros() as usize + 1;
            let rand_idx = rng.gen_bits(lut_bits) as usize;
            let insert = LUT.get_unchecked(rand_idx);
            let insert_len = *LUT_LEN.get_unchecked(rand_idx);
            let insert_vec = vld1q_u8(insert.0.as_ptr());
            vst1q_u8(out_ptr.add(insert_idx), insert_vec);

            // shuffle to shift right by amount only known at runtime
            let rest_vec = shuffle(vec, vaddq_u8(indexes, vdupq_n_u8(insert_idx as u8)));
            vst1q_u8(out_ptr.add(insert_idx + insert_len), rest_vec);
            out_ptr = out_ptr.add(insert_len);
            len += insert_len;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[target_feature(enable = "neon")]
unsafe fn nya_ify_neon(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let bit5 = vdupq_n_u8(0b0010_0000);
    let splat_n = vdupq_n_u8(b'n');
    let splat_space = vdupq_n_u8(b' ');
    let splat_tab = vdupq_n_u8(b'\t');
    let splat_newline = vdupq_n_u8(b'\n');
    let indexes = vld1q_u8(&[0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] as _);

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        let vec = vld1q_u8(in_ptr.add(i));
        let n_mask = vceqq_u8(vorrq_u8(vec, bit5), splat_n);
        let space_mask = vorrq_u8(
            vceqq_u8(vec, splat_space),
            vorrq_u8(vceqq_u8(vec, splat_tab), vceqq_u8(vec, splat_newline)),
        );
        // only nya-ify if its space followed by 'n'
        let space_and_n_mask = vandq_u8(shift_left(space_mask, 1), n_mask);
        let mut nya_mask = movemask(space_and_n_mask) as u32;

        vst1q_u8(out_ptr, vec);

        // try to nya-ify as many as possible in the current vector
        while nya_mask != 0 {
            let nya_idx = nya_mask.trailing_zeros() as usize;
            ptr::write(out_ptr.add(nya_idx + 1), b'y');
            // shuffle to shift by amount only known at runtime
            let shifted = shuffle(vec, vaddq_u8(indexes, vdupq_n_u8(nya_idx as u8 + 1)));
            vst1q_u8(out_ptr.add(nya_idx + 2), shifted);
            out_ptr = out_ptr.add(1);
            len += 1;
            nya_mask &= nya_mask - 1;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[target_feature(enable = "neon")]
unsafe fn replace_and_stutter_neon(
    rng: &mut XorShift32,
    in_bytes: &[u8],
    mut len: usize,
    out_bytes: &mut [u8],
) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let bit5 = vdupq_n_u8(0b0010_0000);
    let splat_backtick = vdupq_n_u8(b'`');
    let splat_open_brace = vdupq_n_u8(b'{');
    let splat_l = vdupq_n_u8(b'l');
    let splat_r = vdupq_n_u8(b'r');
    let splat_w = vdupq_n_u8(b'w');
    let splat_space = vdupq_n_u8(b' ');
    let splat_tab = vdupq_n_u8(b'\t');
    let splat_newline = vdupq_n_u8(b'\n');
    let indexes = vld1q_u8(&[0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] as _);

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        // replace 'l' and 'r' with 'w'
        let vec = vld1q_u8(in_ptr.add(i));
        let vec_but_lower = vorrq_u8(vec, bit5);
        let alpha_mask = vandq_u8(
            vcgtq_u8(vec_but_lower, splat_backtick),
            vcgtq_u8(splat_open_brace, vec_but_lower),
        );
        let replace_mask = vorrq_u8(
            vceqq_u8(vec_but_lower, splat_l),
            vceqq_u8(vec_but_lower, splat_r),
        );
        let replaced = blend(vec_but_lower, splat_w, replace_mask);
        // make sure only alphabetical characters are lowercased and replaced
        let mut res = blend(vec, replaced, alpha_mask);

        // sometimes, add a stutter if there is a space, tab, or newline followed by any letter
        let space_mask = vorrq_u8(
            vceqq_u8(vec, splat_space),
            vorrq_u8(vceqq_u8(vec, splat_tab), vceqq_u8(vec, splat_newline)),
        );
        let space_and_alpha_mask = vandq_u8(shift_left(space_mask, 1), alpha_mask);
        let stutter_mask = movemask(space_and_alpha_mask) as u32;

        vst1q_u8(out_ptr, res);

        if stutter_mask != 0 {
            let stutter_idx = stutter_mask.trailing_zeros() as usize;
            // shuffle to shift by amount only known at runtime
            res = shuffle(res, vaddq_u8(indexes, vdupq_n_u8(stutter_idx as u8)));
            vst1q_u8(out_ptr.add(stutter_idx), vsetq_lane_u8::<1>(b'-', res));
            // decide whether to stutter in a branchless way
            // a branch would mispredict often since this is random
            let increment = if rng.gen_bool() { 2 } else { 0 };
            vst1q_u8(out_ptr.add(stutter_idx + increment), res);
            out_ptr = out_ptr.add(increment);
            len += increment;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    #[test]
    fn test_uwuify_neon() {
        let mut temp_bytes1 = [0u8; 1024];
        let mut temp_bytes2 = [0u8; 1024];

        let s = "Hey, I think I really love you. Do you want a headpat?";
        let res_bytes = uwuify_neon(s.as_bytes(), &mut temp_bytes1, &mut temp_bytes2);
        let res = str::from_utf8(res_bytes).unwrap();
        assert_eq!(
            res,
            "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?"
        );
    }

    #[test]
    fn test_uwuify_str_neon() {
        let s = "Hey, I think I really love you. Do you want a headpat?";
        let res = uwuify_str_neon(s);
        assert_eq!(
            res,
            "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?"
        );
    }
}
