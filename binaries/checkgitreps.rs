use std::process::Command;
use std::io::{self, Write};
use std::fs;
use std::env;

fn main () {
    // Variable env name
    let key = "HOME";

    // Handle env variable error
    let home = match env::var(key) {
        Ok(val) => val,
        Err(_e) => "none".to_string(),
    };

    // Get my repositories folder
    let dir = format!("{home}/reps");

    // For directory entries return DiEntry Result
    for entry in fs::read_dir(&dir).unwrap() {
        // Get the value of the entry Result
        if let Ok(entry) = entry {
            
            let entry_str = match entry.file_name().into_string() {
                Ok(val) => val,
                Err(_e) => "none".to_string(),
            };

            //Format String to command
            let formated = format!("{}/{}", &dir, &entry_str);

            println!("{}", formated);

            // Run git status -C command with formated directory
            let git = { Command::new("git")
            .arg("-C")
            .arg(formated)
            .arg("status")
            .output()
            .expect("failed to execute process")
            };

            // Print the status to stdout
            io::stdout().write_all(&git.stdout).unwrap();
        }
    };

    

}