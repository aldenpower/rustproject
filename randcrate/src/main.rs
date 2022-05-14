// Prelude import only a small essential part of a crate
use rand::prelude::*;


fn main() {

    for _number in 0..10 {

        let x: u8 = random();

        println!("{}", x);
    }

    if random(){
        println!("Heads!");
    }



    let rng = thread_rng();

    println!("{}", rng.gen()::f64);
}
