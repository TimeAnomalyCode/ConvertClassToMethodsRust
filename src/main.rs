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
        let mut public_methods: Vec<String> = vec![];
        let mut isPublicMethod = false;
        for line in content.lines(){
            let new_line = line.trim_start();
            if new_line.starts_with("//") || new_line.is_empty() || new_line.contains("#") || new_line.contains("{") || new_line.contains("}") || new_line.contains("virtual"){
                continue;
            }

            if new_line.contains("class"){
                class_name = new_line.replace("class", "");
            }

            if new_line.starts_with("public:"){
                isPublicMethod = true;
                continue;
            }

            if isPublicMethod {
                if new_line.contains("private:") || new_line.contains("protected:") || new_line.contains("}"){
                    isPublicMethod = false;
                    continue;
                }
                public_methods.push(new_line.to_string());
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

    }

    // println!("{content}");
}
