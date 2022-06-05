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


    for entry in fs::read_dir(&dir).unwrap() {

        if let Ok(entry) = entry {

            let entry_str = match entry.file_name().into_string() {
                Ok(val) => val,
                Err(_e) => "none".to_string(),
            };


            let formated = format!("{}/{}", &dir, &entry_str);

            println!("{}", formated);

            let git = { Command::new("git")
            .arg("-C")
            .arg(formated)
            .arg("status")
            .output()
            .expect("failed to execute process")
            };

            // println!("Repository name: {:?}", entry);
            io::stdout().write_all(&git.stdout).unwrap();
        }
    };

    

}