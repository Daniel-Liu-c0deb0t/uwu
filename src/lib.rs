#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub mod rng;
use rng::XorShift32;

fn round_up(a: usize, b: usize) -> usize { (a + b - 1) / b * b }

pub fn uwu_ify_sse<'a>(bytes: &[u8], mut len: usize, temp_bytes1: &'a mut [u8], temp_bytes2: &'a mut [u8]) -> &'a [u8] {
    assert!(round_up(len, 16) <= bytes.len());
    assert!(temp_bytes1.len() >= bytes.len() * 4);
    assert!(temp_bytes2.len() >= bytes.len() * 4);

    let mut rng = XorShift32::new(b"uwu!");

    unsafe {
        len = replace_and_stutter_sse(&mut rng, bytes, len, temp_bytes1);
        len = emoji_sse(&mut rng, temp_bytes1, len, temp_bytes2);
        &temp_bytes2[..len]
    }
}

#[repr(align(16))]
struct A(&'static [u8; 16]);

struct Insert {
    bytes: A,
    len: usize
}

static LUT: [Insert; 8] = [
    Insert { bytes: A(b" rawr x3        "), len: 8 },
    Insert { bytes: A(b" OwO            "), len: 4 },
    Insert { bytes: A(b" UwU            "), len: 4 },
    Insert { bytes: A(b" o.O            "), len: 4 },
    Insert { bytes: A(b" -.-            "), len: 4 },
    // " 𝓤𝔀𝓤"
    Insert { bytes: A(&[0x20, 0xf0, 0x9d, 0x93, 0xa4, 0xf0, 0x9d, 0x94, 0x80, 0xf0, 0x9d, 0x93, 0xa4, 0, 0, 0]), len: 13 },
    // " (⑅˘꒳˘)"
    Insert { bytes: A(&[0x20, 0x28, 0xe2, 0x91, 0x85, 0xcb, 0x98, 0xea, 0x92, 0xb3, 0xcb, 0x98, 0x29, 0, 0, 0]), len: 13 },
    // " (ꈍᴗꈍ)"
    Insert { bytes: A(&[0x20, 0x28, 0xea, 0x88, 0x8d, 0xe1, 0xb4, 0x97, 0xea, 0x88, 0x8d, 0x29, 0, 0, 0, 0]), len: 12 }
];

unsafe fn emoji_sse(rng: &mut XorShift32, in_bytes: &[u8], mut len: usize, out_bytes: &mut [u8]) -> usize {
    let in_ptr = in_bytes.as_ptr();
    let mut out_ptr = out_bytes.as_mut_ptr();

    let splat_period = _mm_set1_epi8(b'.' as i8);
    let splat_comma = _mm_set1_epi8(b',' as i8);
    let splat_space = _mm_set1_epi8(b' ' as i8);
    let splat_tab = _mm_set1_epi8(b'\t' as i8);
    let splat_newline = _mm_set1_epi8(b'\n' as i8);
    let indexes = _mm_set_epi8(15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0);

    let lut_bits = LUT.len().trailing_zeros() as u32;

    let iter_len = round_up(len, 16);

    for i in (0..iter_len).step_by(16) {
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let mut punctuation_mask = _mm_or_si128(_mm_cmpeq_epi8(vec, splat_comma), _mm_cmpeq_epi8(vec, splat_period));
        let mut multiple_mask = _mm_and_si128(punctuation_mask, _mm_slli_si128(punctuation_mask, 1));
        multiple_mask = _mm_or_si128(multiple_mask, _mm_srli_si128(multiple_mask, 1));
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

        if insert_mask != 0 {
            let insert_idx = insert_mask.trailing_zeros() as usize + 1;
            let insert = LUT.get_unchecked(rng.gen_bits(lut_bits) as usize);
            let insert_vec = _mm_load_si128(insert.bytes.0.as_ptr() as *const __m128i);
            _mm_storeu_si128(out_ptr.add(insert_idx) as *mut __m128i, insert_vec);
            // shuffle to shift by amount only known at runtime
            let rest_vec = _mm_shuffle_epi8(vec, _mm_add_epi8(indexes, _mm_set1_epi8(insert_idx as i8)));
            _mm_storeu_si128(out_ptr.add(insert_idx + insert.len) as *mut __m128i, rest_vec);
            out_ptr = out_ptr.add(insert.len);
            len += insert.len;
        }

        out_ptr = out_ptr.add(16);
    }

    len
}

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

    let iter_len = round_up(len, 16);

    for i in (0..iter_len).step_by(16) {
        // replace 'l' and 'r' with 'w'
        let vec = _mm_loadu_si128(in_ptr.add(i) as *const __m128i);
        let vec_but_lower = _mm_or_si128(vec, bit5);
        let alpha_mask = _mm_and_si128(_mm_cmpgt_epi8(vec_but_lower, splat_backtick), _mm_cmpgt_epi8(splat_open_brace, vec_but_lower));
        let replace_mask = _mm_or_si128(_mm_cmpeq_epi8(vec_but_lower, splat_l), _mm_cmpeq_epi8(vec_but_lower, splat_r));
        let replaced = _mm_blendv_epi8(vec_but_lower, splat_w, replace_mask);
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
            let increment = if rng.gen_bits(2) == 0 { 2 } else { 0 };
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
    fn test_replace_and_stutter_sse() {
        let mut temp_bytes1 = [0u8; 1024];
        let mut temp_bytes2 = [0u8; 1024];

        let mut bytes = "Hello world! blah blah... hi, this is a sentence.".as_bytes().to_owned();
        let len = bytes.len();
        bytes.resize(round_up(len, 16), 0);
        let res_bytes = uwu_ify_sse(&bytes, len, &mut temp_bytes1, &mut temp_bytes2);
        let res = str::from_utf8(res_bytes).unwrap();
        assert_eq!(res, "hewwo w-wowwd! bwah bwah... hi, (⑅˘꒳˘) this is a sentence.");
    }
}
