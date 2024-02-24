# lazyrand wasm sample

How to make wasm project with lazyrand?

## Install crates

install crates:

```sh
# install wasm-pack
cargo install wasm-pack
cargo add wasm-bindgen
# install lazyrand
cargo add lazyrand
```

## Write Cargo.toml

Add Cargo.toml to crate-type.

```toml:Cargo.toml
[package]
name = "wasm_test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
lazyrand = "0.1.12"
```

## Write src/lib.rs

The function is defined as follows. The definition provided is for the case when the randint function is used.

```rust:src/lib.rs
// define for lazyrand
#[wasm_bindgen]
pub fn randint(min: isize, max: isize) -> i64 {
    if lazyrand::get_tag() != 2 { lazyrand::set_seed_plus(get_current_time()); }
    lazyrand::randint(min as i64, max as i64)
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
```

For other definitions, please refer to the actual lib.rs file.

## build

```sh
wasm-pack build --target web --release
```

## Write index.html

```html:index.html
<script type="module">
    // WASMを読み込む
    import init, { randint } from "./pkg/wasm_test.js";
    init().then(() => {
        console.log('randint=', randint(1, 6))
    });
</script>
```

Check index.html in your browser. However, loading WASM must be done via http/https.

