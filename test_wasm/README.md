# lazyrand wasm sample

How to make wasm project with lazyrand?

## Install crates

install crates:

```sh
# create project
cargo init --lib

# install wasm-pack
cargo install wasm-pack
cargo add wasm-bindgen

# install lazyrand
cargo add lazyrand
```

## Write Cargo.toml

Add Cargo.toml to crate-type.

```toml:Cargo.toml
...

[lib]
crate-type = ["cdylib", "rlib"]
...
```

## Write src/lib.rs

The function is defined as follows. The definition provided is for the case when the randint function is used.

```rust:src/lib.rs
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
    import init, { randint, rand } from "./pkg/wasm_test.js";
    init().then(() => {
        console.log('randint=', randint(1, 6))
    });
</script>
```

Check index.html in your browser. However, loading WASM must be done via http/https.

