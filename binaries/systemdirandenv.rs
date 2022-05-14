use std::env;
use std::fs;


fn main () {
    let current_dir = env::current_dir();
    let current_exe = env::current_exe();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }


    println!("{:#?}", current_exe);
}