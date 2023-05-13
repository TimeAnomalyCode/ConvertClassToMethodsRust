use std::fmt::format;
use std::io;
use std::fs;
use std::io::BufReader;
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

        let mut new_cpp_file = fs::File::create(new_cpp);

        let mut class_name = String::new();

        let mut is_public_method = false;
        let mut public_methods: Vec<String> = vec![];
        
        let mut is_class_variable = false;
        let mut class_variables: Vec<String> = vec![];

        for line in content.lines(){
            let new_line = line.trim_start();
            if new_line.starts_with("//") || new_line.is_empty() || new_line.contains("#") || new_line.contains("{") || new_line.contains("}") || new_line.contains("virtual"){
                continue;
            }

            if new_line.contains("class"){
                class_name = new_line.replace("class", "");
            }

            if new_line.starts_with("public:"){
                is_public_method = true;
                continue;
            }

            if is_public_method {
                if new_line.contains("private:") || new_line.contains("protected:") || new_line.contains("}"){
                    is_public_method = false;
                }
                else {
                    public_methods.push(new_line.to_string());
                }
            }

            if new_line.contains("protected:") || new_line.contains("private:"){
                is_class_variable = true;
                continue;
            }

            if is_class_variable {
                if new_line.contains("public:") || new_line.contains("}"){
                    is_class_variable = false;
                }
                else {
                    class_variables.push(new_line.to_string());
                }
            }
            // println!("{}", new_line);
        }

        class_name = class_name.trim().to_owned();

        let template = format!(
            "
            #include <iostream>\n
            #include \"{file_name}\"\n
            \n
            using namespace std;\n
            \n
            ")
            .to_string();

        for method in public_methods{
            if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool"){

                let new_method = method.trim_end_matches(';');
                let signature = new_method.splitn(2, " ").nth(1).unwrap();
                let new_method = format!("{} {}::{}{{\n\n}}", method.split_whitespace().next().unwrap(), class_name, signature);

                println!("{}", new_method);
            }
            else {
                // for constructor/deconstructor
                
                let new_method = method.to_owned();
                let new_method = new_method.trim_end_matches(';');
                let new_method = format!("{}::{}{{\n\n}}", class_name, new_method);

                println!("{}", new_method);
            }
        }

    }

    // println!("{content}");
}
