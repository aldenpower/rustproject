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
[reference](https://riptutorial.com/rust/example/26619/basic-error-reporting-and-handling)

## Some crate cool imports
```rust
use std::process::Command;
use std::env::Args;
use clap::{App, Arg};
use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::fs;
```
## Strings
```rust
//create string
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
```rust
fn main () {
    let mut s: String = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
    s.replace_range(.., "baz");
    println!("{}", s);
}
```
```rust
fn main () {
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
}
```
```rust
fn main () {
    let s1: String = String::from("Hello, ");
    let s2: String = String::from("aaa, ");
    let s4: String = String::from("bafd, ");
    
    let s: String = format!("{}{}{}", s1, s2, s4);
    //let s: String = format!("{}{}{}", s1, s2, "toe");
    println!("{}", s)
}
```
```rust
fn main () {
    let s1: String = ["first", "second"].concat();
    let s2: String = format!("{}{}", "first", "second");
    let s3: &str = concat!("first", "second");
    
    let s4: String = String::from("test");
    let s5: String = s4 + "okok";
}
```
```rust
fn main () {
    // slicing string
    let s1: &str = "akoskd";
    let s2: &str = &s1[1..2];
    println!("{}", s2);
}
```
```rust
// Iterate string
fn main () {
    for b in "dasd".bytes() {
        println!("{}", b);
    }
    
    for b in "dasd".bytes() {
        println!("{}", b);
    }
    
    fn main () {
    for b in "dasd".graphemes(true) {
        println!("{}", b);
    }
}
}

```rust
fn main () {
    let s1: &str = "Hello world!";
    let s2: String = String::from("Hello world!");
    my_function(s1);
    // pass reference string will automatically use string slice
    my_function(&s2);
}

// -> String takes ownership of the string
fn my_function (a: &str) -> String {
    return format!("{}", a);
}
```
## I/O
``` rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
let weight: f32 = input.trim().parse().unwrap();
```

## .unwrap()
If the **result** is **ok** it will return the ok value,
if not ok terminate the program and the error is returned to the screen

## Comments
> All variables in Rust are immutable by default

> References can be mutable or immutable

> We can have multiple and immutable references at the same time

> Each value in Rust is owned by a variable.

> When the owner goes out of scope, the value will be deallocated

> There can only be ONE owner at a time

## Future sections
- The manifest Format
