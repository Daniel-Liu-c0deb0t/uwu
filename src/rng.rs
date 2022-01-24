// xorshift32
// literally one of the simplest and fastest RNGs
// augmented with a simple counter like "xorwow"

pub struct XorShift32 {
    state: u32,
    counter: u32,
}

impl XorShift32 {
    #[inline(always)]
    pub fn new(seed: &[u8; 4]) -> Self {
        let mut state = 0u32;
        state |= seed[0] as u32;
        state |= (seed[1] as u32) << 8;
        state |= (seed[2] as u32) << 16;
        state |= (seed[3] as u32) << 24;
        XorShift32 {
            state: state | 1,
            counter: state,
        }
    }

    #[inline(always)]
    pub fn gen_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.counter = self.counter.wrapping_add(1234567891u32);
        self.state.wrapping_add(self.counter)
    }

    #[inline(always)]
    pub fn gen_bits(&mut self, bits: u32) -> u32 {
        self.gen_u32() & ((1 << bits) - 1)
    }

    #[inline(always)]
    pub fn gen_bool(&mut self) -> bool {
        // kinda wasteful but ok
        self.gen_bits(1) > 0
    }
}
