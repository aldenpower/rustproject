use std::process::Command;
use std::env::args;
use clap::{App, Arg};

fn main() {
    let matches = App::new("echo")
        .version("0.1.0")
        .author("aldenpower <eng.andersonfsl@gmail.com>")
        .about("rust echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
            .help("Do not print newline")
            .takes_value(false)
            .short('n'),
        )
        .get_matches();



    println!("{:#?}", matches);

}


