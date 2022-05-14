fn main() {

    // const needs the data type defined
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadow variable
    let k = 12;
    let k = k + 1; // This variable is shadowed by the first

    // Shadowing can change for string to num
    let spaces = "   ";
    let spaces = spaces.len();

    // Compiling error the first spaces has string type
    //let mut spaces = "  ";
    //spaces = spaces.len();

    // INTEGERS

    // Signed variables (Accept negative values)
    // Length 8-bit | signed (i8) | unsigned (u8)
    // Length 16-bit | signed (i16) | unsigned (u16)
    // Length 32-bit | signed (i32) | unsigned (u32)
    // Length 64-bit | signed (i64) | unsigned (u64)
    // Length 128-bit | signed (i128) | unsigned (u128)
    // Length arch-bit | signed (isize) | unsigned (usize)
    //
    // arch-bit depends on the pc architecture
    
    // Formula for stored number in variables
    // -(2^(n-1)) to (2^(n-1))-1 
    // n is the number of bits that variant uses
    
    // You can separate numbers with _
    // 1000 = 1_000

    let n = 57u8;
    let n: u8 = 57;

    // FLOAT
    // let x = 2.0 //f64
    // let y: f32 = 3.0 //f32

    // Boolean
    // let t = true;
    // let f: bool = false;

    // Char
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';
    
    // Operations
    // 5 + 10
    // 95 - 4.3
    // 4 * 30
    // 56.7 / 32.2
    // 2 / 3 results in 0
    // 43 % 5
    

    // Rust has two primitive types: tuples and arrays
    // The tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0; // indexing

    println!("{}", five_hundred);

    // The array type
    // Cannot change the size
    let a = [1, 2, 3, 4, 5]; //must have the same type
    // always contains same value
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // array with 5 threes

    let first = a[0]; // indexing

}
