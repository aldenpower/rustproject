fn main() {
    println!("Hello, world!");
    another_function(2_0, 'a');

    let x = five();
    println!("The value of x is : {}", x);

    let x = plus_one(5);

    println!("The value of x is : {}", x);

}

//In functions you must declare the type of each parameter
fn another_function(x: u8, y: char){
    println!("Another function");
    println!("{} {}", x, y);
}


//statement x expressions
//statements dont evaluate to a value
//let x = 22; //this is a statement

//let y = {   //this is a expression, the value x + 1 is a expression (no semicolon)
//    let x = 3;
//    x + 1
//};

//basically a statement doesnt return anything
// the expression returns, a functions is a expression
// If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value


//Return values
fn five() -> i32 {
    5 //no semicolon value turn into a expression
}


fn plus_one(x: i32) -> i32 {
    x + 1 // ; put a semicolon generate error
}
