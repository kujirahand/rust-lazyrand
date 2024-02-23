//!
//! xoshiro256pp
//! (ref) <https://prng.di.unimi.it/xoshiro256plusplus.c>
//!

pub struct Xoroshiro256pp {
    pub s: [u64; 4],
}

impl Xoroshiro256pp {
    pub fn new() -> Self {
        Self { s: [0; 4] }
    }
    pub fn from_seed(seed: u64) -> Self {
        let mut s = Self { s: [0; 4] };
        s.set_seed(seed);
        s
    }
    pub fn jump(&mut self) {
        let jump_params: [u64; 4] = [
            0x180ec6d33cfd0aba,
            0xd5a61266f0c9392c,
            0xa9582618e03fc9aa,
            0x39abdc4529b1661c,
        ];
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for i in 0..jump_params.len() {
            for b in 0..64 {
                if (jump_params[i] & (1 << b)) != 0 {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next();
            }
        }
        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }

    pub fn long_jump(&mut self) {
        let jump_params: [u64; 4] = [
            0x76e15d3efefdcbbf,
            0xc5004e441c522fb3,
            0x77710069854ee241,
            0x39109bb02acbe635,
        ];
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for i in 0..jump_params.len() {
            for b in 0..64 {
                if (jump_params[i] & (1 << b)) != 0 {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next();
            }
        }
        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }
    pub fn set_seed(&mut self, seed: u64) {
        let seed = seed ^ 16868548727063204;
        self.s[0] = seed >> 0 & 0xFFFF;
        self.s[1] = seed >> 16 & 0xFFFF;
        self.s[2] = seed >> 32 & 0xFFFF;
        self.s[3] = seed >> 48 & 0xFFFF;
        self.jump();
    }

    pub fn next(&mut self) -> u64 {
        let added = self.s[0].wrapping_add(self.s[3]);
        let result = rtol(added, 23).wrapping_add(self.s[0]);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = rtol(self.s[3], 45);

        result
    }
}

fn rtol(x: u64, k: usize) -> u64 {
    (x << k) | (x >> (64 - k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut rng = Xoroshiro256pp::from_seed(0);
        let a = rng.next();
        let b = rng.next();
        let c = rng.next();
        assert_ne!(a, b);
        assert_ne!(b, c);
    }
}
