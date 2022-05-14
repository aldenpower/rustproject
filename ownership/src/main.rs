fn main() {
    // println!("{}", s);

    // let mut s = String::from("hello");

    // s.push_str(", world!");

    // let s = String::from("geek");

    // println!("{}", calculate_lenght(&s));

    // change(&s);


    fn calculate_lenght(s: &String) -> usize { //s is a reference to a String
        s.len()
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world")
    }

    // MUTABLE REFERENCE
    let mut k = String::from("hello");
    change(&mut k);

}
