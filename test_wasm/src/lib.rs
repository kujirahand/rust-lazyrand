use wasm_bindgen::prelude::*;

// defeine for lazyrand
#[wasm_bindgen]
pub fn set_seed(seed: u64)  {
    lazyrand::set_seed(seed)
}
#[wasm_bindgen]
pub fn randint(min: isize, max: isize) -> i64 {
    if lazyrand::get_tag() != 2 { lazyrand::set_seed_plus(get_current_time()); }
    lazyrand::randint(min as i64, max as i64)
}
#[wasm_bindgen]
pub fn rand() -> u64 {
    if lazyrand::get_tag() != 2 { lazyrand::set_seed_plus(get_current_time()); }
    lazyrand::rand()
}

// for get_current_time
#[wasm_bindgen]
extern "C" {
    type Date;
    #[wasm_bindgen(constructor)]
    fn new() -> Date;
    #[wasm_bindgen(method)]
    fn getTime(this: &Date) -> f64;
}
#[wasm_bindgen]
pub fn get_current_time() -> u64 {
    let date = Date::new();
    (date.getTime() * 1000.0) as u64
}
