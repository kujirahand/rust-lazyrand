# lazyrand for Rust

It is a library for generating random numbers easily.

It is a library that can be used for various purposes such as games and dice.
But this library is not cryptographically secure.

The random seed is automatically initialized.

- [crates.io](https://crates.io/crates/lazyrand)
- [docs.rs](https://docs.rs/lazyrand)

## How to install

Run the following command in your terminal:

```sh
cargo add lazyrand
```

## Examples

Generate random number

```rust
let num = lazyrand::randint(1, 6);
println!("random number = {}", num);
```

Generate random number with seed.
It can be used to generate the same random number sequence.

```rust
lazyrand::set_seed(123456);
let n1 = lazyrand::rand() % 6 + 1;
let n2 = lazyrand::rand() % 6 + 1;
let n3 = lazyrand::rand() % 6 + 1;
println!("nums = [{}, {}, {}]", n1, n2, n3);
```

## Examples - Slice

Shuffle slice

```rust
let mut a = vec![1, 2, 3, 4, 5];
lazyrand::shuffle(&mut a);
println!("shuffled = {:?}", a); // (ex) shuffled = [2, 4, 1, 5, 3]
```

choice one number from slice

```rust
let a = vec![1, 2, 3];
let n = lazyrand::choice(&a);
println!("choice = {:?}", n); // (ex) choice = Some(5)
```

choice one &str from slice

```rust
let mut a = vec!["apple", "banana", "orange"];
let s = lazyrand::choice(&a).unwrap();
println!("choice = {}", s); // (ex) choice = orange
```

## Examples with Random struct

Generate random number with Random struct

```rust
use lazyrand::Random;
fn main() {
    // Create Random generator
    let mut random = Random::new();
    println!("random number = {}", random.randint(1, 6));

    let msg: &str = random.choice(&["hello", "hi", "bye"]).unwrap();
    println!("random message = {}", msg);
}
```

## How to Use Lazyrand with WASM

Please see [test_wasm](test_wasm/README.md).

## methods

- set_seed(seed) / srand(seed) / generate_seed()
- randint(min, max)
- shuffle(&mut slice)
- choice(&slice)
- rand() / rand_bool() / rand_usize() / rand_isize() / rand_f64()


