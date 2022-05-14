mod structs;
mod functions;


use structs::{Cow, Milk};

fn main () {
    compra6_1v1b();
}

fn compra6_1v1b () {
    let temp1 = [
        String::from("2022-01-05"),
        String::from("2022-05-08")
    ];
    let milk1 = Milk {price : 1.5, temp : temp1};

    let cow1 = Cow {
        name: String::from("6a"),
        valor_compra : -7000.0,
        valor_venda : 0.0,
        capacity : 8,
        milk : vec![milk1]
    };

    println!("{:?}", &cow1.milk);
    println!("{:?}", &cow1.spread());

}