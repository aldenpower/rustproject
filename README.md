# Rust study

## Creating a library and link it to another crate

_Create a file called **rary.rs** and put some library code inside_
```rust
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}
```
_Compile with **--crate-type** flag_

```bash
rustc --crate-type=lib rary.rs
```

> Libraries get prefixed with "lib", and by default they get named after their crate file, but this default name can be overridden by passing the --crate-name option to rustc or by using the crate_name attribute.

_Compile the executable using a library code inside than execute the file_

```bash
rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable
```

[reference](https://doc.rust-lang.org/rust-by-example/crates.html)

## Some crate cool imports
```rust
use std::process::Command;
use std::env::Args;
use clap::{App, Arg};
```
## Strings
```rust
fn main () {
    let s1: &str = "😁";
    let s2: String = String::from("doakoask");
    let s3: String = "dkaos".to_string();
    let s4: String = "dkaos".to_owned();
    let s5: &str = &s4[0..2];
    let s6: &str = &s4[..];
    
    println!("{}", s5)
}
```

## Future sections
- The manifest Format
