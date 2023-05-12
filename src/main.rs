use std::io;
use std::fs;
use std::io::Write;

fn main() {

    let mut file_path = String::new();

    print!("Enter filePath: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to Read Line");
    
    let file_path = file_path.trim();

    let content = fs::read_to_string(file_path)
        .expect("Failed To Read File");

    println!("{content}");
}
