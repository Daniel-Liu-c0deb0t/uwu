#![cfg(any(target_arch = "arm", target_arch = "aarch64"))]

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;

use super::{bytes_len, str_to_bytes, A};

pub struct Bitap8x16 {
    v: int32x4_t,
    start_mask: uint8x16_t,
}

const fn get_masks(patterns: &[&str]) -> [A; 256] {
    // preprocessing step to associate each character with a mask of locations
    // in each of the 8 pattern strings

    // must use const to init this array
    const TEMP_A: A = A([0u8; 16]);
    let mut res = [TEMP_A; 256];
    let mut i = 0;
    let bit5 = 0b0010_0000u8;

    while i < patterns.len() {
        let bytes = patterns[i].as_bytes();
        // offset masks so the last character maps to the last bit of each 16-bit lane
        // this is useful for movemask later
        let offset = 16 - bytes.len();
        let mut j = 0;

        while j < bytes.len() {
            let idx = i * 16 + j + offset;
            res[bytes[j] as usize].0[idx / 8] |= 1u8 << (idx % 8);

            // make sure to be case insensitive
            if bytes[j].is_ascii_alphabetic() {
                res[(bytes[j] ^ bit5) as usize].0[idx / 8] |= 1u8 << (idx % 8);
            }

            j += 1;
        }

        i += 1;
    }

    res
}

const fn get_start_mask(patterns: &[&str]) -> A {
    // get a mask that indicates the first character for each pattern
    let mut res = A([0u8; 16]);
    let mut i = 0;

    while i < patterns.len() {
        let j = 16 - patterns[i].as_bytes().len();
        let idx = i * 16 + j;
        res.0[idx / 8] |= 1u8 << (idx % 8);
        i += 1;
    }

    res
}

static PATTERNS: [&str; 8] = [
    "small", "cute", "fluff", "love", "stupid", "what", "meow", "meow",
];

static MASKS: [A; 256] = get_masks(&PATTERNS);
static START_MASK: A = get_start_mask(&PATTERNS);

// important note: replacement cannot be more than 2 times longer than the corresponding pattern!
// this is to prevent increasing the size of the output too much in certain cases
// another note: this table has a fixed size of 8 and expanding it will require changing the
// algorithm a little
static REPLACE: [A; 8] = [
    str_to_bytes("smol"),
    str_to_bytes("kawaii~"),
    str_to_bytes("floof"),
    str_to_bytes("luv"),
    str_to_bytes("baka"),
    str_to_bytes("nani"),
    str_to_bytes("nya~"),
    str_to_bytes("nya~"),
];

const fn get_len(a: &[A]) -> [usize; 8] {
    let mut res = [0usize; 8];
    let mut i = 0;

    while i < a.len() {
        res[i] = bytes_len(&a[i].0);
        i += 1;
    }

    res
}

static REPLACE_LEN: [usize; 8] = get_len(&REPLACE);

#[inline(always)]
pub unsafe fn movemask(a: uint8x16_t) -> i32 {
    let high_bits = vreinterpretq_u16_u8(vshrq_n_u8::<7>(a));
    let paired16 = vreinterpretq_u32_u16(vsraq_n_u16::<7>(high_bits, high_bits));
    let paired32 = vreinterpretq_u64_u32(vsraq_n_u32::<14>(paired16, paired16));
    let paired64 = vreinterpretq_u8_u64(vsraq_n_u64::<28>(paired32, paired32));
    vgetq_lane_u8::<0>(paired64) as i32 | ((vgetq_lane_u8::<8>(paired64) as i32) << 8)
}

#[derive(Debug, PartialEq)]
pub struct Match {
    pub match_len: usize,
    pub replace_ptr: *const int32x4_t,
    pub replace_len: usize,
}

impl Bitap8x16 {
    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn new() -> Self {
        Self {
            v: vdupq_n_s32(0),
            start_mask: vld1q_u8(START_MASK.0.as_ptr()),
        }
    }

    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn next(&mut self, c: u8) -> Option<Match> {
        self.v = vreinterpretq_s32_s16(vshlq_n_s16::<1>(vreinterpretq_s16_s32(self.v)));
        self.v = vorrq_s32(self.v, vreinterpretq_s32_u8(self.start_mask));
        let mask = vld1q_u8(MASKS.get_unchecked(c as usize).0.as_ptr());
        self.v = vandq_s32(self.v, vreinterpretq_s32_u8(mask));

        let match_mask = (movemask(vreinterpretq_u8_s32(self.v)) as u32) & 0xAAAAAAAAu32;

        if match_mask != 0 {
            let match_idx = (match_mask.trailing_zeros() as usize) / 2;

            return Some(Match {
                match_len: PATTERNS.get_unchecked(match_idx).len(),
                replace_ptr: REPLACE.get_unchecked(match_idx).0.as_ptr() as *const int32x4_t,
                replace_len: *REPLACE_LEN.get_unchecked(match_idx),
            });
        }

        None
    }

    #[inline]
    #[target_feature(enable = "neon")]
    pub unsafe fn reset(&mut self) {
        self.v = vdupq_n_s32(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitap() {
        unsafe {
            let mut b = Bitap8x16::new();
            assert_eq!(b.next(b'c'), None);
            assert_eq!(b.next(b'u'), None);
            assert_eq!(b.next(b't'), None);
            let next = b.next(b'e').unwrap();
            assert_eq!(next.match_len, 4);
            assert_eq!(next.replace_len, 7);

            b.reset();
            assert_eq!(b.next(b'w'), None);
            assert_eq!(b.next(b'h'), None);
            assert_eq!(b.next(b'a'), None);
            let next = b.next(b't').unwrap();
            assert_eq!(next.match_len, 4);
            assert_eq!(next.replace_len, 4);

            assert_eq!(b.next(b'w'), None);
            assert_eq!(b.next(b'h'), None);
            assert_eq!(b.next(b'a'), None);
            assert_eq!(b.next(b'a'), None);

            assert_eq!(b.next(b'W'), None);
            assert_eq!(b.next(b'h'), None);
            assert_eq!(b.next(b'A'), None);
            let next = b.next(b't').unwrap();
            assert_eq!(next.match_len, 4);
            assert_eq!(next.replace_len, 4);
        }
    }
}
