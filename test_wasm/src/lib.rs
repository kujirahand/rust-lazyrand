use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_randint(min: i64, max: i64) -> i64 {
    lazyrand::randint(min, max)
}
