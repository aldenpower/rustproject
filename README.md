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