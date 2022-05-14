fn main() {
    // String is a collection of bytes
    // String::from and to_string do the same thing

    let mut s = String::new();

    let data = "initial contents";

    // let s = String::from("initial contents");

    s = data.to_string();
    // let s = "initial contents".to_string();

    s.push_str(" bar");

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // works like println! but returns String instead of print
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Apropriatte way for indexing strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}


