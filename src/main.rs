use std::fs::OpenOptions;
use std::io;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let file_path = Path::new(r"test\templateClass.h");
    let file_content = fs::read_to_string(file_path).expect("File does not Exist");

    let public_methods = get_template_class(file_content);
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

fn get_private_methods(file_content: String) -> Vec<String>{
    let mut private_methods: Vec<String> = vec![];
    let mut is_private_methods = false;
    for line in file_content.lines(){
        let line = line.trim_start();

        if line.starts_with("//") || line.is_empty() || line.contains("#") || line.contains("};") || line.contains("virtual"){
            continue;
        }

        if line.starts_with("private:"){
            is_private_methods = true;
            continue;
        }

        if line.starts_with("public:") || line.starts_with("protected:"){
            is_private_methods = false;
            continue;
        }

        if is_private_methods{
            private_methods.push(line.to_string());
        }
    }

    return private_methods;
}

fn get_template_class(file_content: String)-> String {
    let mut template_name = String::new();
    // let mut template_class: Vec<String> = vec![];
    let mut is_template_class = false;
    for line in file_content.lines(){
        let line = line.trim_start();

        if line.starts_with("//") || line.is_empty() || line.contains("#") || line.contains("};") || line.contains("virtual"){
            continue;
        }

        if line.starts_with("template"){
            is_template_class = true;
        }

        if is_template_class {
            template_name = line.to_string();

            break;
        }
    }

    return template_name;
}

fn get_file_path_from_user()-> String {
    let mut file_path = String::new();

    print!("Enter filePath: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to Read Line");
    
    let file_path = file_path.trim().to_string();

    return file_path;
}

