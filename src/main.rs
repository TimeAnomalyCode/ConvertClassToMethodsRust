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
        let new_cpp_name_path = file_path.file_name().unwrap().to_str().unwrap();
        let new_cpp_name_list: Vec<_> = new_cpp_name_path.split('.').collect();
        let new_cpp_name = new_cpp_name_list[0];
        let new_cpp_path = file_path.parent().unwrap().to_str().unwrap();

        let new_cpp = new_cpp_path.to_owned() + "/" + new_cpp_name + ".cpp";
        let mut new_cpp_file = fs::File::create(new_cpp);
    }

    // println!("{content}");
}
