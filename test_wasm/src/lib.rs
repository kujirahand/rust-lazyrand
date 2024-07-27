use wasm_bindgen::prelude::*;

/// return random value
#[wasm_bindgen]
pub fn randint(min: isize, max: isize) -> i64 {
    lazyrand::randint(min as i64, max as i64)
}
/// return random value
#[wasm_bindgen]
pub fn rand() -> u64 {
    lazyrand::rand()
}
/// initialize random seed
#[wasm_bindgen]
pub fn init_rand() -> u64 {
    let t = lazyrand::lazyrand_time::get_init_seed();
    lazyrand::set_seed_plus(t);
    t
}
/// manually set seed
#[wasm_bindgen]
pub fn set_seed(seed: u64) {
    lazyrand::set_seed_plus(seed);
}
