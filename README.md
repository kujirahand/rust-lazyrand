# lazyrand for Rust

It is a simple library for generating random numbers easily.
Random numbers are automatically initialized.
But this library is not cryptographically secure.

# How to install

Run the following command in your terminal:

```install.sh
cargo add lazyrand
```

# Examples

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

# Examples - Shuffle

Shuffle slice

```
let mut a = vec![1, 2, 3, 4, 5];
lazyrand::shuffle(&mut a);
println!("shuffled = {:?}", a);
```

# Examples with Random struct

Generate random number with Random struct

```
let mut random = lazyrand::Random::new();
println!("random number = {}", random.randint(1, 6));
```
