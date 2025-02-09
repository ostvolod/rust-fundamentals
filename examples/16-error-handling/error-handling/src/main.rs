use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();

    if arguments.len() != 2 {
        println!("Usage: {} <filename>", arguments[0]);
        return;
    }

    let path = &arguments[1];
        if path.is_empty() {
        println!("Filename cannot be empty");
        return;
    }

    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
