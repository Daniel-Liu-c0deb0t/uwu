//! fastest text uwuifier in the west

#![allow(clippy::missing_safety_doc)]

pub mod bitap;
pub mod bitap_neon;
pub mod rng;
mod uwuify_neon;
mod uwuify_sse;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use uwuify_sse::{uwuify_sse as uwuify, uwuify_str_sse as uwuify_str};

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub use uwuify_neon::{uwuify_neon as uwuify, uwuify_str_neon as uwuify_str};

#[repr(align(16))]
struct A([u8; 16]);

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

// this lookup table needs to be power of two sized
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
    str_to_bytes(" (✿oωo)"),
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

/// round up `n` to the next multiple of 16. useful for allocating buffers
///
/// # example:
/// ```
/// use uwuifier::round_up16;
/// assert_eq!(round_up16(17), 32);
/// ```
#[inline(always)]
pub fn round_up16(n: usize) -> usize {
    (n + 15) / 16 * 16
}

#[inline(always)]
fn pad_zeros(bytes: &mut [u8], len: usize) {
    for i in len..round_up16(len) {
        unsafe {
            *bytes.get_unchecked_mut(i) = 0u8;
        }
    }
}
