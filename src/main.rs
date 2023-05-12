use std::io;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {

    let mut file_path = String::new();

    print!("Enter filePath: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to Read Line");
    
    let file_path = Path::new(file_path.trim());

    let content = fs::read_to_string(file_path)
        .expect("Failed To Read File");

    if !content.contains("template"){
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let new_cpp_name = Path::new(file_name).file_stem().unwrap().to_str().unwrap();
        let new_cpp_path = file_path.parent().unwrap().to_str().unwrap();
        let new_cpp = format!("{}/{}.cpp", new_cpp_path, new_cpp_name);

        let new_cpp_file = fs::File::create(new_cpp);
    }

    // println!("{content}");
}
