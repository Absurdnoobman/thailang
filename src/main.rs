use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Replace "your_file.txt" with the actual file path
    let file_path = "your_file.txt";

    match read_file(file_path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    
    file.read_to_string(&mut content)?;

    Ok(content)
}

