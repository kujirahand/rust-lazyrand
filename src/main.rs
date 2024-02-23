// This is a simple example of using lazyrand.

use lazyrand::Random;
fn main() {
    // randint
    for _ in 0..5 {
        println!("{}", lazyrand::randint(1, 6));
    }
    println!("---");
    // shuffle
    let mut a = vec![1, 2, 3, 4, 5];
    lazyrand::shuffle(&mut a);
    println!("shuffled = {:?}", a); // (example)
    println!("---");
    // choice
    let a = vec![1, 2, 3, 4, 5];
    let n = lazyrand::choice(&a);
    println!("choice = {:?}", n);
    println!("---");
    // choice &str
    let a = vec!["apple", "banana", "orange"];
    let s = lazyrand::choice(&a).unwrap();
    println!("choice = {}", s); // (ex) orange
    println!("---");
    // random struct
    let mut random = Random::new();
    for _ in 0..5 {
        println!("{}", random.randint(1, 6));
    }
    println!("---");
    for _ in 0..5 {
        println!("{}", random.rand());
    }

}
