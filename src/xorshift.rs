//!
//! XorShift random number generator
//!
//!
static SEED_STATE_XOR_VALUE: u64 = 132366047211908; // for seed randomize

pub struct XorShift64 {
    state: u64,
}
impl XorShift64 {
    pub fn new() -> Self {
        Self { state: 0 }
    }
    /// crete random generator with seed
    pub fn from_seed(seed: u64) -> Self {
        let mut state = Self::new();
        state.set_seed(seed);
        state
    }
    pub fn set_seed(&mut self, seed: u64) {
        self.state = seed ^ SEED_STATE_XOR_VALUE;
    }
    /// generate random number by xorshift64 algorithm
    pub fn next(&mut self) -> u64 {
        let (a, b, c) = (13, 7, 17);
        let mut x = self.state;
        x ^= x << a;
        x ^= x >> b;
        x ^= x << c;
        self.state = x;
        self.state
    }
}

/// generate random number by xorshift64 algorithm
pub fn xorshift64(state: &mut u64) -> u64 {
    let (a, b, c) = (13, 7, 17);
    let mut x = *state;
    x ^= x << a;
    x ^= x >> b;
    x ^= x << c;
    *state = x;
    *state
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xorshift() {
        let mut random = XorShift64::from_seed(123456);
        let result = xorshift64(&mut random.state);
        assert_eq!(result, 8689614632028771299);
        let result = xorshift64(&mut random.state);
        assert_eq!(result, 9134513685019898372);
    }
}
