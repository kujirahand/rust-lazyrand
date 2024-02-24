//!
//! It is a simple library for generating random numbers easily.
//! The random seed is automatically initialized.
//! But this library is not cryptographically secure.
//! It is based on xoshiro256plusplus.
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
//! let f = lazyrand::rand_f64();
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
use std::thread;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use xoshiro256pp::Xoroshiro256pp;

// mod xorshift;
mod xoshiro256pp;

pub struct Random {
    pub gen: Xoroshiro256pp,
    pub tag: u32, // tag=0: not initialized/1: auto generated seed/2: user specified seed
}
impl Random {
    /// create random generator by current time
    pub fn new() -> Self {
        let gen = Xoroshiro256pp::from_seed(Self::gen_seed());
        Self { gen, tag: 1 } // tag=1 ... auto generated seed
    }

    /// generate seed by current time and thread id
    pub fn gen_seed() -> u64 {
        generate_seed()
    }

    /// create random generator with seed
    pub fn from_seed(seed: u64) -> Self {
        let gen = Xoroshiro256pp::from_seed(seed);
        Self { gen, tag: 2 } // tag=2 ... user specified seed
    }

    /// set random seed
    pub fn set_seed(&mut self, seed: u64) {
        self.tag = 2;
        self.gen.set_seed(seed);
    }
    /// generate random number in range [0, u64::MAX]
    pub fn rand(&mut self) -> u64 {
        self.gen.next()
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
    pub fn rand_bool(&mut self) -> bool {
        (self.rand() % 2) == 1
    }
    /// generate random float in range 0.0 < 1.0
    pub fn rand_f64(&mut self) -> f64 {
        (1.0 / (u64::MAX as f64)) * (self.rand() as f64)
    }
    /// generate random number as isize
    pub fn rand_isize(&mut self) -> isize {
        self.rand() as isize
    }
    /// generate random number as usize
    pub fn rand_usize(&mut self) -> usize {
        self.rand() as usize
    }
}

static RANDOM: Lazy<Mutex<Random>> = Lazy::new(|| Mutex::new(Random::new()));

/// get tag
pub fn get_tag() -> u32 {
    RANDOM.lock().unwrap().tag
}

/// set tag
pub fn set_tag(tag: u32) {
    RANDOM.lock().unwrap().tag = tag;
}

/// set random seed
pub fn srand(seed: u64) {
    RANDOM.lock().unwrap().set_seed(seed);
}

/// set random seed
pub fn set_seed(seed: u64) {
    RANDOM.lock().unwrap().set_seed(seed);
}

/// set random seed
pub fn set_seed_plus(seed: u64) {
    let seed_plus = generate_seed() ^ seed;
    RANDOM.lock().unwrap().set_seed(seed_plus);
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
pub fn rand_bool() -> bool {
    RANDOM.lock().unwrap().rand_bool()
}

/// generate random float in range 0.0 < 1.0
pub fn rand_f64() -> f64 {
    RANDOM.lock().unwrap().rand_f64()
}

/// generate random isize
pub fn rand_isize() -> isize {
    RANDOM.lock().unwrap().rand_isize()
}

/// generate random usize
pub fn rand_usize() -> usize {
    RANDOM.lock().unwrap().rand_usize()
}

/// for WebAssembly (#1)
#[cfg(target_arch = "wasm32")]
fn get_time_msec() -> u64 {
    10164339691474454771
}
#[cfg(not(target_arch = "wasm32"))]
/// get current time in milliseconds
fn get_time_msec() -> u64 {
    let now = std::time::SystemTime::now();
    match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_micros() as u64,
        Err(_) => 0,
    }
}
/// get local variable address
pub fn get_var_addr() -> u64 {
    let var = 0x1234567;
    let var_ptr: *const u64 = &var;
    let addr: u64 = var_ptr as u64;
    if addr == 0 { var } else { addr }
}
/// get seed from address and time
fn get_seed_from_addr_n_time() -> u64 {
    get_var_addr() ^ get_time_msec()
}
/// generate seed by current time and thread id
pub fn generate_seed() -> u64 {
    // generate seed by current time and thread id
    let mut hasher = DefaultHasher::new();
    hasher.write_u64(get_seed_from_addr_n_time());
    thread::current().id().hash(&mut hasher);
    let hash = hasher.finish();
    hash
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
    fn test_from_seed() {
        let mut rnd = Random::from_seed(123456);
        let a1 = rnd.rand();
        let a2 = rnd.rand();
        let a3 = rnd.rand();
        let mut rnd2 = Random::from_seed(123456);
        let b1 = rnd2.rand();
        let b2 = rnd2.rand();
        let b3 = rnd2.rand();
        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
        assert_eq!(a3, b3);
    }
    #[test]
    fn test_randint() {
        let mut random = Random::from_seed(123456);
        for _ in 0..=1000 {
            let v = random.randint(0, 9);
            assert!(v >= 0 && v <= 9);
        }
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
        assert!(v[0] > 450 && v[0] < 550);
        //
        srand(123456);
        let mut v = vec![0, 0];
        for _ in 0..1000 {
            let r = randint(0, 1);
            v[r as usize] += 1;
        }
        assert!(v[0] > 450 && v[0] < 550);
    }
    #[test]
    fn test_shuffle() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut random = Random::from_seed(111);
        random.shuffle(&mut a);
        random.shuffle(&mut b);
        assert_ne!(a, b);
    }
    #[test]
    fn test_choice() {
        let mut random = Random::from_seed(222);
        // choice number
        let a = vec![1, 2, 3, 4, 5];
        let val = random.choice(&a).unwrap();
        assert!(val >= 1 && val <= 5);
        //
        let a = vec![1, 2];
        let val = random.choice(&a).unwrap();
        assert!(val >= 1 && val <= 2);
        //
        let a: Vec<usize> = vec![];
        let res = random.choice(&a);
        assert_eq!(res, None);

        // choice &str
        let a = vec!["banana"];
        let val = random.choice(&a).unwrap();
        assert_eq!(val, "banana");
    }
    #[test]
    fn test_rand_bool() {
        srand(123456);
        let mut v = [0, 0];
        for _ in 0..1000 {
            let r = rand_bool();
            let r = if r { 1 } else { 0 };
            v[r as usize] += 1;
        }
        println!("v={:?}", v);
        assert!(v[0] > 400 && v[0] < 600);
    }
    #[test]
    fn test_randf64() {
        srand(123456);
        for _ in 0..1000 {
            let r = rand_f64();
            assert!(r >= 0.0 && r < 1.0);
        }
    }
    #[test]
    fn test_rand_seed() {
        let seed = Random::gen_seed();
        assert!(seed != 0);
    }

}
