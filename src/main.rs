use std::fs::OpenOptions;
use std::io;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let file_path = Path::new(r"test\templateClass.h");
    let file_content = fs::read_to_string(file_path).expect("File does not Exist");

    let public_methods = get_public_methods(file_content);
    println!("{:?}", public_methods);
}

fn get_public_methods(file_content: String) -> Vec<String>{
    let mut public_methods: Vec<String> = vec![];
    let mut is_public_methods = false;
    for line in file_content.lines(){
        let line = line.trim_start();

        if line.starts_with("//") || line.is_empty() || line.contains("#") || line.contains("};") || line.contains("virtual"){
            continue;
        }

        if line.starts_with("public:"){
            is_public_methods = true;
            continue;
        }

        if line.starts_with("private:") || line.starts_with("protected:"){
            is_public_methods = false;
            continue;
        }

        if is_public_methods{
            public_methods.push(line.to_string());
        }
    }

    return public_methods;
}
