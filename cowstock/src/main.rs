mod structs;

use structs::{Cow, Milk};

fn main () {
    compra6_1v1b();
}

fn compra6_1v1b () {
    let temp1 = [
        String::from("2022-01-05"),
        String::from("2022-05-08")
    ];
    let milk = Milk {price : 1.5, temp : temp1};

    let cow = Cow::new(
        "6a".to_string(),
        -7000.0,
        0.0,
        8,
        vec![milk]
    );
   
    //dbg!(&cow);
    println!("{:?}", cow.m);
    println!("{:?}", cow.spread());
}