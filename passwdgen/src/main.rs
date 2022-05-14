mod args;

use args::MyArgs;
use clap::Parser;

fn main() {
    let args = MyArgs::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}