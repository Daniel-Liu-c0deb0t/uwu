//! fastest text uwuifier in the west
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::{ptr, str};

pub mod rng;
use rng::XorShift32;

pub mod bitap;
use bitap::Bitap8x16;

#[repr(align(16))]
struct A([u8; 16]);

/// round up `n` to the next multiple of 16. useful for allocating buffers
///
/// # example:
/// ```
/// use uwuifier::round_up16;
/// assert_eq!(round_up16(17), 32);
/// ```
#[inline(always)]
pub fn round_up16(n: usize) -> usize { (n + 15) / 16 * 16 }

#[inline(always)]
fn pad_zeros(bytes: &mut [u8], len: usize) {
    for i in len..round_up16(len) {
        unsafe { *bytes.get_unchecked_mut(i) = 0u8; }
    }
}

/// uwuify a string slice
///
/// requires the sse4.1 x86 feature
///
/// this is probably fine for one-off use, but not very efficient if called multiple times.
/// use `uwuify_sse` to reduce memory allocations
///
/// # example:
/// ```
/// use uwuifier::uwuify_str_sse;
/// assert_eq!(uwuify_str_sse("hello world"), "hewwo wowwd");
/// ```
pub fn uwuify_str_sse(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut temp1 = vec![0u8; round_up16(bytes.len()) * 16];
    let mut temp2 = vec![0u8; round_up16(bytes.len()) * 16];
    unsafe { str::from_utf8_unchecked(uwuify_sse(bytes, &mut temp1, &mut temp2)).to_owned() }
}

/// uwuify some bytes
///
/// requires the sse4.1 x86 feature
///
/// `temp_bytes1` and `temp_bytes2` must be buffers of size `round_up16(bytes.len()) * 16`,
/// because this is the worst-case size of the output. yes, it is annoying to allocate by
/// hand, but simd :)
///
/// the returned slice is the uwu'd result. when working with utf-8 strings, just pass in
/// the string as raw bytes and convert the output slice back to a string afterwards.
/// there's also the `uwuify_str_sse` function that is suitable for one-off use with a string slice
///
/// # example:
/// ```
/// use uwuifier::{uwuify_sse, round_up16};
/// let s = "hello world";
/// let b = s.as_bytes();
/// let mut temp1 = vec![0u8; round_up16(b.len()) * 16];
/// let mut temp2 = vec![0u8; round_up16(b.len()) * 16];
/// let res = uwuify_sse(b, &mut temp1, &mut temp2);
/// assert_eq!(std::str::from_utf8(res).unwrap(), "hewwo wowwd");
/// ```
pub fn uwuify_sse<'a>(bytes: &[u8], temp_bytes1: &'a mut [u8], temp_bytes2: &'a mut [u8]) -> &'a [u8] {
    if !is_x86_feature_detected!("sse4.1") {
        panic!("sse4.1 feature not detected!");
    }
    assert!(temp_bytes1.len() >= round_up16(bytes.len()) * 16);
    assert!(temp_bytes2.len() >= round_up16(bytes.len()) * 16);

    // only the highest quality seed will do
    let mut rng = XorShift32::new(b"uwu!");

    let mut len = bytes.len();

    unsafe {
        // bitap_sse will not read past len, unlike the other passes
        len = bitap_sse(bytes, len, temp_bytes1);
        pad_zeros(temp_bytes1, len);
        len = nya_ify_sse(temp_bytes1, len, temp_bytes2);
        pad_zeros(temp_bytes2, len);
        len = replace_and_stutter_sse(&mut rng, temp_bytes2, len, temp_bytes1);
        pad_zeros(temp_bytes1, len);
        len = emoji_sse(&mut rng, temp_bytes1, len, temp_bytes2);
        &temp_bytes2[..len]
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn bitap_sse(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let mut out_ptr = out_bytes.as_mut_ptr();
    let mut bitap = Bitap8x16::new();
    let iter_len = len;

    for i in 0..iter_len {
        let c = *in_bytes.get_unchecked(i);
        ptr::write(out_ptr, c);
        out_ptr = out_ptr.add(1);

        if let Some(m) = bitap.next(c) {
            let replace = _mm_load_si128(m.replace_ptr);
            _mm_storeu_si128(out_ptr.sub(m.match_len) as *mut __m128i, replace);
            out_ptr = out_ptr.add(m.replace_len).sub(m.match_len);
            len = len + m.replace_len - m.match_len;
            bitap.reset();
        }
    }

    len
}

const fn str_to_bytes(s: &str) -> A {
    let bytes = s.as_bytes();
    let mut res = A([0u8; 16]);
    let mut i = 0;

    while i < bytes.len() {
        res.0[i] = bytes[i];
        i += 1;
    }

    res
}

const LUT_SIZE: usize = 32;
static LUT: [A; LUT_SIZE] = [
    str_to_bytes(" rawr x3"),
    str_to_bytes(" OwO"),
    str_to_bytes(" UwU"),
    str_to_bytes(" o.O"),
    str_to_bytes(" -.-"),
    str_to_bytes(" >w<"),
    str_to_bytes(" (⑅˘꒳˘)"),
    str_to_bytes(" (ꈍᴗꈍ)"),
    str_to_bytes(" (˘ω˘)"),
    str_to_bytes(" (U ᵕ U❁)"),
    str_to_bytes(" σωσ"),
    str_to_bytes(" òωó"),
    str_to_bytes(" (///ˬ///✿)"),
    str_to_bytes(" (U ﹏ U)"),
    str_to_bytes(" ( ͡o ω ͡o )"),
    str_to_bytes(" ʘwʘ"),
    str_to_bytes(" :3"),
    str_to_bytes(" :3"), // important enough to have twice
    str_to_bytes(" XD"),
    str_to_bytes(" nyaa~~"),
    str_to_bytes(" mya"),
    str_to_bytes(" >_<"),
    str_to_bytes(" 😳"),
    str_to_bytes(" 🥺"),
    str_to_bytes(" 😳😳😳"),
    str_to_bytes(" rawr"),
    str_to_bytes(" ^^"),
    str_to_bytes(" ^^;;"),
    str_to_bytes(" (ˆ ﻌ ˆ)♡"),
    str_to_bytes(" ^•ﻌ•^"),
    str_to_bytes(" /(^•ω•^)"),
    str_to_bytes(" (✿oωo)")
];

const fn bytes_len(b: &[u8]) -> usize {
    let mut len = 0;

    while len < b.len() && b[len] != 0 {
        len += 1;
    }

    len
}

const fn get_len(a: &[A]) -> [usize; LUT_SIZE] {
    let mut res = [0usize; LUT_SIZE];
    let mut i = 0;

    while i < a.len() {
        res[i] = bytes_len(&a[i].0);
        i += 1;
    }

    res
}

static LUT_LEN: [usize; LUT_SIZE] = get_len(&LUT);

#[target_feature(enable = "sse4.1")]
unsafe fn emoji_sse(rng: &mut XorShift32, in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let splat_period = _mm_set1_epi8(b'.' as i8);
    let splat_comma = _mm_set1_epi8(b',' as i8);
    let splat_exclamation = _mm_set1_epi8(b'!' as i8);
    let splat_space = _mm_set1_epi8(b' ' as i8);
    let splat_tab = _mm_set1_epi8(b'\t' as i8);
    let splat_newline = _mm_set1_epi8(b'\n' as i8);
    let indexes = _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    let lut_bits = LUT.len().trailing_zeros() as u32;

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let mut punctuation_mask = _mm_or_si128(
            _mm_cmpeq_epi8(vec, splat_comma),
            _mm_or_si128(_mm_cmpeq_epi8(vec, splat_period), _mm_cmpeq_epi8(vec, splat_exclamation))
        );
        // multiple punctuation in a row means no emoji
        let mut multiple_mask = _mm_and_si128(punctuation_mask, _mm_slli_si128(punctuation_mask, 1));
        multiple_mask = _mm_or_si128(multiple_mask, _mm_srli_si128(multiple_mask, 1));
        // punctuation must be followed by a space or else no emoji
        let space_mask = _mm_or_si128(
            _mm_cmpeq_epi8(vec, splat_space),
            _mm_or_si128(_mm_cmpeq_epi8(vec, splat_tab), _mm_cmpeq_epi8(vec, splat_newline))
        );
        punctuation_mask = _mm_andnot_si128(
            multiple_mask,
            _mm_and_si128(punctuation_mask, _mm_srli_si128(space_mask, 1))
        );
        let insert_mask = _mm_movemask_epi8(punctuation_mask) as u32;

        _mm_storeu_si128(out_ptr as *mut __m128i, vec);

        // be lazy and only allow one emoji per vector
        if insert_mask != 0 {
            let insert_idx = insert_mask.trailing_zeros() as usize + 1;
            let rand_idx = rng.gen_bits(lut_bits) as usize;
            let insert = LUT.get_unchecked(rand_idx);
            let insert_len = *LUT_LEN.get_unchecked(rand_idx);
            let insert_vec = _mm_load_si128(insert.0.as_ptr() as *const __m128i);
            _mm_storeu_si128(out_ptr.add(insert_idx) as *mut __m128i, insert_vec);

            // shuffle to shift right by amount only known at runtime
            let rest_vec = _mm_shuffle_epi8(vec, _mm_add_epi8(indexes, _mm_set1_epi8(insert_idx as i8)));
            _mm_storeu_si128(out_ptr.add(insert_idx + insert_len) as *mut __m128i, rest_vec);
            out_ptr = out_ptr.add(insert_len);
            len += insert_len;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[target_feature(enable = "sse4.1")]
unsafe fn nya_ify_sse(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let bit5 = _mm_set1_epi8(0b0010_0000);
    let splat_n = _mm_set1_epi8(b'n' as i8);
    let splat_space = _mm_set1_epi8(b' ' as i8);
    let splat_tab = _mm_set1_epi8(b'\t' as i8);
    let splat_newline = _mm_set1_epi8(b'\n' as i8);
    let indexes = _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let n_mask = _mm_cmpeq_epi8(_mm_or_si128(vec, bit5), splat_n);
        let space_mask = _mm_or_si128(
            _mm_cmpeq_epi8(vec, splat_space),
            _mm_or_si128(_mm_cmpeq_epi8(vec, splat_tab), _mm_cmpeq_epi8(vec, splat_newline))
        );
        // only nya-ify if its space followed by 'n'
        let space_and_n_mask = _mm_and_si128(_mm_slli_si128(space_mask, 1), n_mask);
        let mut nya_mask = _mm_movemask_epi8(space_and_n_mask) as u32;

        _mm_storeu_si128(out_ptr as *mut __m128i, vec);

        // try to nya-ify as many as possible in the current vector
        while nya_mask != 0 {
            let nya_idx = nya_mask.trailing_zeros() as usize;
            ptr::write(out_ptr.add(nya_idx + 1), b'y');
            // shuffle to shift by amount only known at runtime
            let shifted = _mm_shuffle_epi8(vec, _mm_add_epi8(indexes, _mm_set1_epi8(nya_idx as i8 + 1)));
            _mm_storeu_si128(out_ptr.add(nya_idx + 2) as *mut __m128i, shifted);
            out_ptr = out_ptr.add(1);
            len += 1;
            nya_mask &= nya_mask - 1;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[target_feature(enable = "sse4.1")]
unsafe fn replace_and_stutter_sse(rng: &mut XorShift32, in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let bit5 = _mm_set1_epi8(0b0010_0000);
    let splat_backtick = _mm_set1_epi8(b'`' as i8);
    let splat_open_brace = _mm_set1_epi8(b'{' as i8);
    let splat_l = _mm_set1_epi8(b'l' as i8);
    let splat_r = _mm_set1_epi8(b'r' as i8);
    let splat_w = _mm_set1_epi8(b'w' as i8);
    let splat_space = _mm_set1_epi8(b' ' as i8);
    let splat_tab = _mm_set1_epi8(b'\t' as i8);
    let splat_newline = _mm_set1_epi8(b'\n' as i8);
    let indexes = _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    let iter_len = round_up16(len);

    for i in (0..iter_len).step_by(16) {
        // replace 'l' and 'r' with 'w'
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let vec_but_lower = _mm_or_si128(vec, bit5);
        let alpha_mask = _mm_and_si128(_mm_cmpgt_epi8(vec_but_lower, splat_backtick), _mm_cmpgt_epi8(splat_open_brace, vec_but_lower));
        let replace_mask = _mm_or_si128(_mm_cmpeq_epi8(vec_but_lower, splat_l), _mm_cmpeq_epi8(vec_but_lower, splat_r));
        let replaced = _mm_blendv_epi8(vec_but_lower, splat_w, replace_mask);
        // make sure only alphabetical characters are lowercased and replaced
        let mut res = _mm_blendv_epi8(vec, replaced, alpha_mask);

        // sometimes, add a stutter if there is a space, tab, or newline followed by any letter
        let space_mask = _mm_or_si128(
            _mm_cmpeq_epi8(vec, splat_space),
            _mm_or_si128(_mm_cmpeq_epi8(vec, splat_tab), _mm_cmpeq_epi8(vec, splat_newline))
        );
        let space_and_alpha_mask = _mm_and_si128(_mm_slli_si128(space_mask, 1), alpha_mask);
        let stutter_mask = _mm_movemask_epi8(space_and_alpha_mask) as u32;

        _mm_storeu_si128(out_ptr as *mut __m128i, res);

        if stutter_mask != 0 {
            let stutter_idx = stutter_mask.trailing_zeros() as usize;
            // shuffle to shift by amount only known at runtime
            res = _mm_shuffle_epi8(res, _mm_add_epi8(indexes, _mm_set1_epi8(stutter_idx as i8)));
            _mm_storeu_si128(out_ptr.add(stutter_idx) as *mut __m128i, _mm_insert_epi8(res, b'-' as i32, 1));
            // decide whether to stutter in a branchless way
            // a branch would mispredict often since this is random
            let increment = if rng.gen_bool() { 2 } else { 0 };
            _mm_storeu_si128(out_ptr.add(stutter_idx + increment) as *mut __m128i, res);
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
    fn test_uwuify_sse() {
        let mut temp_bytes1 = [0u8; 1024];
        let mut temp_bytes2 = [0u8; 1024];

        let s = "Hey... I think I really love you. Do you want a headpat?";
        let res_bytes = uwuify_sse(s.as_bytes(), &mut temp_bytes1, &mut temp_bytes2);
        let res = str::from_utf8(res_bytes).unwrap();
        assert_eq!(res, "hey... i think i w-weawwy wuv you. (⑅˘꒳˘) d-do you want a headpat?");
    }

    #[test]
    fn test_uwuify_str_sse() {
        let s = "Hey... I think I really love you. Do you want a headpat?";
        let res = uwuify_str_sse(s);
        assert_eq!(res, "hey... i think i w-weawwy wuv you. (⑅˘꒳˘) d-do you want a headpat?");
    }
}
