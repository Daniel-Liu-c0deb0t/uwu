#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use super::{A, str_to_bytes, bytes_len};

pub struct Bitap8x16 {
    v: __m128i,
    start_mask: __m128i
}

const fn get_masks(patterns: &[&str]) -> [A; 256] {
    // preprecessing step to associate each character with a mask of locations
    // in each of the 8 pattern strings

    // must use const to init this array
    const TEMP_A: A = A([0u8; 16]);
    let mut res = [TEMP_A; 256];
    let mut i = 0;

    while i < patterns.len() {
        let bytes = patterns[i].as_bytes();
        // offset masks so the last character maps to the last bit of each 16-bit lane
        // this is useful for movemask later
        let offset = 16 - bytes.len();
        let mut j = 0;

        while j < bytes.len() {
            let idx = i * 16 + j + offset;
            res[bytes[j] as usize].0[idx / 8] |= 1u8 << (idx % 8);
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
    "small",
    "cute",
    "fluff",
    "love",
    "stupid",
    "what",
    "meow",
    "meow"
];

static MASKS: [A; 256] = get_masks(&PATTERNS);
static START_MASK: A = get_start_mask(&PATTERNS);

// important note: replacement cannot be more than 2 times longer than the corresponding pattern!
// this is to prevent increasing the size of the output too much in certain cases
static REPLACE: [A; 8] = [
    str_to_bytes("smol"),
    str_to_bytes("kawaii~"),
    str_to_bytes("floof"),
    str_to_bytes("luv"),
    str_to_bytes("baka"),
    str_to_bytes("nani"),
    str_to_bytes("nya~"),
    str_to_bytes("nya~")
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

#[derive(Debug, PartialEq)]
pub struct Match {
    pub match_len: usize,
    pub replace_ptr: *const __m128i,
    pub replace_len: usize
}

impl Bitap8x16 {
    #[inline]
    #[target_feature(enable = "sse4.1")]
    pub unsafe fn new() -> Self {
        Self {
            v: _mm_setzero_si128(),
            start_mask: _mm_load_si128(START_MASK.0.as_ptr() as *const __m128i)
        }
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    pub unsafe fn next(&mut self, c: u8) -> Option<Match> {
        self.v = _mm_slli_epi16(self.v, 1);
        self.v = _mm_or_si128(self.v, self.start_mask);
        let mask = _mm_load_si128(MASKS.get_unchecked(c as usize).0.as_ptr() as *const __m128i);
        self.v = _mm_and_si128(self.v, mask);

        let match_mask = (_mm_movemask_epi8(self.v) as u32) & 0xAAAAAAAAu32;

        if match_mask != 0 {
            let match_idx = (match_mask.trailing_zeros() as usize) / 2;

            return Some(Match {
                match_len: PATTERNS.get_unchecked(match_idx).len(),
                replace_ptr: REPLACE.get_unchecked(match_idx).0.as_ptr() as *const __m128i,
                replace_len: *REPLACE_LEN.get_unchecked(match_idx)
            });
        }

        None
    }

    #[inline]
    #[target_feature(enable = "sse4.1")]
    pub unsafe fn reset(&mut self) {
        self.v = _mm_setzero_si128();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitap() {
        if !is_x86_feature_detected!("sse4.1") {
            panic!("sse4.1 feature not detected!");
        }

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
        }
    }
}
