use std::process::Command;
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::fs;


fn main () {
    /*
    Run a command and get the stdout and sterr
    -
    -
    */

    let out = { 

    Command::new("/bin/cat")
        .arg("/home/aldenpower/reps/cur/main/saldo.py")
        .output()
        .expect("failed to execute process")

    };

    io::stdout().write_all(&out.stdout).unwrap();
    io::stderr().write_all(&out.stderr).unwrap();

}