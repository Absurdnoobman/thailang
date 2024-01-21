pub mod lexer;

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Replace "your_file.txt" with the actual file path
    let path: &str = "lang.thai";

    match read_file(path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file: File = File::open(file_path)?;
    let mut content: String = String::new();
    
    file.read_to_string(&mut content)?;

    Ok(content)
}

