use std::process::Command;
use std::fs;
use std::io::{self, Write};


fn main () {
    /*
    Run command
    get the command output and write to file
    */
    let out = {
        Command::new("/usr/bin/python3")
            .arg("./print.py")
            .output()
            .expect("failed to execute process")
    };

    let mut file = fs::File::create("HELLO!.txt").expect("create failed");

    file.write_all(&out.stdout).unwrap();
}