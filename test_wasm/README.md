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
lazyrand = "0.1.10"
```

## Write src/lib.rs

```rs:src/lib.rs
#[wasm_bindgen]
pub fn wasm_randint(min: i64, max: i64) -> i64 {
    lazyrand::randint(min, max)
}
```

## build

```sh
wasm-pack build --target web --release
```

## Write index.html

```html:index.html
<script type="module">
    // WASMを読み込む
    import init, { wasm_rand } from "./pkg/wasm_test.js";
    init().then(() => {
        console.log('rand=', wasm_randint(1, 6))
    });
</script>
```

Check index.html in your browser. However, loading WASM must be done via http/https.

