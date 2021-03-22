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
    const TEMP_A: A = A([0u8; 16]);
    let mut res = [TEMP_A; 256];
    let mut i = 0;

    while i < patterns.len() {
        let bytes = patterns[i].as_bytes();
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
    "what",
    "what"
];

static MASKS: [A; 256] = get_masks(&PATTERNS);
static START_MASK: A = get_start_mask(&PATTERNS);

static REPLACE: [A; 8] = [
    str_to_bytes("smol"),
    str_to_bytes("cute"),
    str_to_bytes("floof"),
    str_to_bytes("daisuki"),
    str_to_bytes("baka"),
    str_to_bytes("nani"),
    str_to_bytes("nani"),
    str_to_bytes("nani")
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

pub struct Match {
    pub match_len: usize,
    pub replace_ptr: *const __m128i,
    pub replace_len: usize
}

impl Bitap8x16 {
    #[inline(always)]
    pub unsafe fn new() -> Self {
        Self {
            v: _mm_setzero_si128(),
            start_mask: _mm_load_si128(START_MASK.0.as_ptr() as *const __m128i)
        }
    }

    #[inline(always)]
    pub unsafe fn next(&mut self, c: u8) -> Option<Match> {
        let mask = _mm_load_si128(MASKS.get_unchecked(c as usize).0.as_ptr() as *const __m128i);
        self.v = _mm_slli_epi16(self.v, 1);
        self.v = _mm_or_si128(self.v, self.start_mask);
        self.v = _mm_and_si128(self.v, mask);

        let match_mask = (_mm_movemask_epi8(mask) as u32) & 0xAAAAAAAAu32;

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

    #[inline(always)]
    pub unsafe fn reset(&mut self) {
        self.v = _mm_setzero_si128();
    }
}
