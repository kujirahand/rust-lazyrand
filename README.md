# lazyrand for Rust

It is a library for generating random numbers easily.

It is a library that can be used for various purposes such as games and dice.
But this library is not cryptographically secure.

The random seed is automatically initialized.

- [crates.io](https://crates.io/crates/lazyrand)
- [docs.rs](https://docs.rs/lazyrand)

## How to install

Run the following command in your terminal:

```install.sh
cargo add lazyrand
```

## Examples

Generate random number

```
let num = lazyrand::randint(1, 6);
println!("random number = {}", num);
```

Generate random number with seed.
It can be used to generate the same random number sequence.

```
lazyrand::srand(123456);
let n1 = lazyrand::rand();
let n2 = lazyrand::rand();
let n3 = lazyrand::rand();
println!("nums = [{}, {}, {}]", n1, n2, n3);
```

## Examples - Shuffle

Shuffle slice

```
let mut a = vec![1, 2, 3, 4, 5];
lazyrand::shuffle(&mut a);
println!("shuffled = {:?}", a);
```

## Examples with Random struct

Generate random number with Random struct

```
let mut random = lazyrand::Random::new();
println!("random number = {}", random.randint(1, 6));

let mut a = vec![1, 2, 3, 4, 5];
random.shuffle(&mut a);
println!("shuffled = {:?}", a);
```

## methods

- srand(seed)
- rand()
- randint(min, max)
- shuffle(&mut slice)

