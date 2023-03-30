//!
//! It is a simple library for generating random numbers easily.
//! The random seed is automatically initialized.
//! But this library is not cryptographically secure.
//!
//! # Examples
//!
//! Generate random number
//!
//! ```
//! let num = lazyrand::randint(1, 6);
//! println!("random number = {}", num);
//! ```
//!
//! Generate random number with seed.
//! It can be used to generate the same random number sequence.
//!
//! ```
//! lazyrand::set_seed(123456);
//! let n1 = lazyrand::rand();
//! let n2 = lazyrand::rand();
//! let n3 = lazyrand::rand();
//! println!("nums = [{}, {}, {}]", n1, n2, n3);
//! ```
//!
//! Generate random floating point number
//!
//! ```
//! let f = lazyrand::randf64();
//! println!("num = {}", f);
//! ```
//!
//! # Examples - Shuffle
//!
//! Shuffle slice
//!
//! ```
//! let mut a = vec![1, 2, 3, 4, 5];
//! lazyrand::shuffle(&mut a);
//! println!("shuffled = {:?}", a);
//! ```
//!
//!  Choice one element from slice
//!
//! ```
//! // choice one number from slice
//! let mut a = vec![1, 2, 3];
//! let n = lazyrand::choice(&a);
//! println!("choice = {:?}", n);
//! // choice one &str from slice
//! let mut a = vec!["apple", "banana", "orange"];
//! let s = lazyrand::choice(&a);
//! println!("choice = {:?}", s);
//! ```
//!
//! # Examples with Random struct
//!
//! Generate random number with Random struct
//!
//! ```
//! let mut random = lazyrand::Random::new();
//! println!("random number = {}", random.randint(1, 6));
//! ```
//!
//!

use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::time::SystemTime;

const SEED_STATE_XOR_VALUE: u64 = 132366047211908; // for seed randomize
pub struct Random {
    seed: u64,
}
impl Random {
    /// create random generator by current time
    pub fn new() -> Self {
        let mut s = Self { seed: 0 };
        s.set_seed(get_time_msec());
        s
    }
    /// create random generator with seed
    pub fn from_seed(seed: u64) -> Self {
        let mut s = Self { seed: 0 };
        s.set_seed(seed);
        s
    }
    /// set random seed
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed ^ SEED_STATE_XOR_VALUE;
    }
    /// generate random number in range [0, u64::MAX]
    pub fn rand(&mut self) -> u64 {
        xorshift64(&mut self.seed)
    }
    /// generate random number in range [min, max]
    pub fn randint(&mut self, min: i64, max: i64) -> i64 {
        let range = max - min + 1;
        let r = self.rand() % range as u64;
        (r as i64) + min
    }
    /// shuffle slice
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut last = slice.len() - 1;
        while last >= 1 {
            let r = (self.rand() % (last as u64)) as usize;
            slice.swap(last, r);
            last -= 1;
        }
    }
    /// pick up one element from slice
    pub fn choice<T: std::clone::Clone>(&mut self, slice: &[T]) -> Option<T> {
        if slice.is_empty() {
            return None;
        }
        let r = (self.rand() % (slice.len() as u64)) as usize;
        Some(slice[r].clone())
    }
    /// generate random bool
    pub fn randbool(&mut self) -> bool {
        (self.rand() % 2) == 1
    }
    /// generate random float in range 0.0 < 1.0
    pub fn randf64(&mut self) -> f64 {
        (1.0 / (u64::MAX as f64)) * (self.rand() as f64)
    }
}

static RANDOM: Lazy<Mutex<Random>> = Lazy::new(|| Mutex::new(Random::new()));

/// set random seed
pub fn srand(seed: u64) {
    RANDOM.lock().unwrap().set_seed(seed);
}

/// set random seed
pub fn set_seed(seed: u64) {
    RANDOM.lock().unwrap().set_seed(seed);
}

/// generate random number in range [0, u64::MAX]
pub fn rand() -> u64 {
    RANDOM.lock().unwrap().rand()
}

/// generate random number in range [min, max]
#[allow(dead_code)]
pub fn randint(min: i64, max: i64) -> i64 {
    RANDOM.lock().unwrap().randint(min, max)
}

/// generate random value true or false
pub fn randbool() -> bool {
    RANDOM.lock().unwrap().randbool()
}

/// generate random float in range 0.0 < 1.0
pub fn randf64() -> f64 {
    RANDOM.lock().unwrap().randf64()
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

/// get current time in milliseconds
pub fn get_time_msec() -> u64 {
    let now = SystemTime::now();
    return match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_micros() as u64,
        Err(_) => 0,
    };
}

/// shuffle slice
pub fn shuffle<T>(slice: &mut [T]) {
    RANDOM.lock().unwrap().shuffle(slice)
}

/// pick up one element from slice
pub fn choice<T: std::clone::Clone>(slice: &[T]) -> Option<T> {
    RANDOM.lock().unwrap().choice(slice)
}

/*
pub fn xorshift32(state: &mut u32) -> u32 {
    // (3, 13, 7), (5, 13, 6), (9, 11, 19)
    let (a, b, c) = (3, 13, 7);
    let mut x = *state;
    x ^= x << a;
    x ^= x >> b;
    x ^= x << c;
    *state = x;
    state
}

pub fn xorshift128(state: &mut u128) -> u128 {
    // (11, 8, 19)
    let (a, b, c) = (11, 8, 19);
    let mut x = *state;
    x ^= x << a;
    x ^= x >> b;
    x ^= x << c;
    *state = x;
    state
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state() {
        let mut seed = Random::from_seed(123456);
        assert_eq!(seed.randint(-10, 10), -5);
        assert_eq!(seed.rand(), 9134513685019898372);
    }
    #[test]
    fn test_xorshift() {
        let mut seed = Random::from_seed(123456);
        let result = xorshift64(&mut seed.seed);
        assert_eq!(result, 8689614632028771299);
        let result = xorshift64(&mut seed.seed);
        assert_eq!(result, 9134513685019898372);
    }
    #[test]
    fn test_rand() {
        let mut random = Random::from_seed(123456);
        let result = random.rand();
        assert_eq!(result, 8689614632028771299);
        let result = random.rand();
        assert_eq!(result, 9134513685019898372);
    }
    #[test]
    fn test_randint() {
        let mut random = Random::from_seed(123456);
        let mut a = vec![];
        for _ in 0..=4 {
            a.push(random.randint(0, 9));
        }
        assert_eq!(a, vec![9, 2, 4, 4, 6]);
    }
    #[test]
    fn test_randint2() {
        // 1000 times
        for _ in 0..1000 {
            let r = randint(5, 15);
            assert!(r >= 5 && r <= 15);
        }
    }
    #[test]
    fn test_rand_bias() {
        srand(99999);
        let mut v = vec![0, 0];
        for _ in 0..1000 {
            let r = randint(0, 1);
            v[r as usize] += 1;
        }
        assert!(v[0] > 400 && v[0] < 600);
        //
        srand(123456);
        let mut v = vec![0, 0];
        for _ in 0..1000 {
            let r = randint(0, 1);
            v[r as usize] += 1;
        }
        assert!(v[0] > 400 && v[0] < 600);
    }
    #[test]
    fn test_shuffle() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut random = Random::from_seed(111);
        random.shuffle(&mut a);
        assert_eq!(a, vec![2, 4, 5, 3, 1]);
    }
    #[test]
    fn test_choice() {
        let mut random = Random::from_seed(222);
        // choice number
        let a = vec![1, 2, 3, 4, 5];
        let val = random.choice(&a).unwrap();
        assert_eq!(val, 5);
        let val = random.choice(&a).unwrap();
        assert_eq!(val, 3);
        // choice &str
        let a = vec!["banana", "mango", "orange", "apple", "grape"];
        let val = random.choice(&a).unwrap();
        assert_eq!(val, "apple");
        let val = random.choice(&a).unwrap();
        assert_eq!(val, "grape");
        // choice String
        let a = vec!["banana".to_string(), "orange".to_string()];
        let val = random.choice(&a).unwrap();
        assert_eq!(val, "banana".to_string());
    }
    #[test]
    fn test_rand_bool() {
        srand(123456);
        let mut v = [0, 0];
        for _ in 0..1000 {
            let r = randbool();
            let r = if r { 1 } else { 0 };
            v[r as usize] += 1;
        }
        assert!(v[0] > 400 && v[0] < 600);
    }
    #[test]
    fn test_randf64() {
        srand(123456);
        for _ in 0..1000 {
            let r = randf64();
            assert!(r >= 0.0 && r < 1.0);
        }
    }
}
