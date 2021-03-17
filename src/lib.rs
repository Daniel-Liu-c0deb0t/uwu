#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::ptr;

pub fn uwu_ify_sse<'a>(bytes: &[u8], mut len: usize, temp_bytes1: &'a mut [u8], temp_bytes2: &'a mut [u8]) -> &'a [u8] {
    assert!(len <= bytes.len());
    assert!(bytes.len() % 16 == 0);

    unsafe {
        len = replace_and_stutter_sse(bytes, len, &mut temp_bytes1);
        len = punctuation_sse(temp_bytes1, len, &mut temp_bytes2);
        temp_bytes2[..len]
    }
}

pub unsafe fn punctuation_sse(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) {

}

pub unsafe fn replace_and_stutter_sse(in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let bit5 = _mm_set1_epi8(0b0010_0000);
    let splat_backtick = _mm_set1_epi8(b'`' as i8);
    let splat_open_brace = _mm_set1_epi8(b'{' as i8);
    let splat_l = _mm_set1_epi8(b'l' as i8);
    let splat_r = _mm_set1_epi8(b'r' as i8);
    let splat_w = _mm_set1_epi8(b'w' as i8);
    let splat_space = _mm_set1_epi8(b' ' as i8);
    let indexes = _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    for i in (0..in_bytes.len()).step_by(16) {
        // replace 'l' and 'r' with 'w'
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let vec_but_lower = _mm_or_si128(vec, bit5);
        let alpha_mask = _mm_or_si128(_mm_cmpgt_epi8(vec_but_lower, splat_backtick), _mm_cmpgt_epi8(splat_open_brace, vec_but_lower));
        let replace_mask = _mm_or_si128(_mm_cmpeq_epi8(vec_but_lower, splat_l), _mm_cmpeq_epi8(vec_but_lower, splat_r));
        let replaced = _mm_blendv_epi8(vec_but_lower, splat_w, replace_mask);
        let mut res = _mm_blendv_epi8(vec, replaced, alpha_mask);

        // sometimes, add a stutter if there is a ' ' followed by any letter
        let space_mask = _mm_cmpeq_epi8(vec, splat_space);
        let space_and_alpha_mask = _mm_and_si128(_mm_slli_si128(space_mask, 1), alpha_mask);
        let stutter_mask = _mm_movemask_epi8(space_and_alpha_mask) as u32;

        _mm_storeu_si128(out_ptr as *mut __m128i, res);

        if stutter_mask != 0 {
            let stutter_idx = stutter_mask.trailing_zeros() as usize;
            ptr::write(out_ptr.add(stutter_idx), *in_ptr.add(i + stutter_idx));
            ptr::write(out_ptr.add(stutter_idx + 1), b'-');
            // shuffle to shift by amount only known at runtime
            res = _mm_shuffle_epi8(res, _mm_add_epi8(indexes, _mm_set1_epi8(stutter_idx as i8)));
            _mm_storeu_si128(out_ptr.add(stutter_idx + 2) as *mut __m128i, res);
            out_ptr = out_ptr.add(2);
            len += 2;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

#[cfg(test)]
mod tests {
    use std::str;

    use super::*;

    fn round_up(a: usize, b: usize) -> usize { (a + b - 1) / b * b }

    #[test]
    fn test_replace_and_stutter_sse() {
        let mut temp_bytes1 = Vec::with_capacity(1024);
        let mut temp_bytes2 = Vec::with_capacity(1024);

        unsafe {
            temp_bytes1.set_len(temp_bytes1.capacity());
            temp_bytes2.set_len(temp_bytes2.capacity());
        }

        let mut bytes = "Hello world!".as_bytes().to_owned();
        let len = bytes.len();
        bytes.resize(round_up(len, 16), 0);
        let res_bytes = uwu_ify_sse(&bytes, len);
        let res = str::from_utf8(res_bytes).unwrap();
        assert_eq!(res, "hewwo w-wowwd!");

        let mut bytes = "hi my name is daniel, plz be nice to me i like to watch anime".as_bytes().to_owned();
        let len = bytes.len();
        bytes.resize(round_up(len, 16), 0);
        let res_bytes = uwu_ify_sse(&bytes, len);
        let res = str::from_utf8(res_bytes).unwrap();
        assert_eq!(res, "hi m-my name is daniew, p-pwz be nice t-to me i wike to w-watch anime");
    }
}
