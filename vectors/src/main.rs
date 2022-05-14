fn main() {
    // Create new vector
    // let v: Vec<i32> = Vec::new();

    // Create new vector with macro
    // let k = vec![1, 2, 3];

    let mut vec2 = Vec::new();
    let vec = vec![0; 5];

    // Push values to vektor
    vec2.push("asd");
    vec2.push("gsafd");

    println!("{:#?}", &vec2);

    // Read elements
    let a = vec![1, 2, 14, 4, 5];
    let third: &i32 = &a[2];
    println!("The third element is {}", third);

    // This will cause the program to panic because 100 reference doesnt exist
    // let does_not_exist = &a[100];
    // Return None without panic
    let does_not_exist = a.get(100);

    // Iterate through elements
     for i in &a {
        println!("{}", i);
    }

    //Change values in vector, using *
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", &v);

    // Use enum to store multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Vector only store one type of value, with enum
    // you can store the same type enum with other values
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];



}

