use std::fs::OpenOptions;
use std::io;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::env;

fn main() {
    // let file_path = Path::new(r"test\arrayListTypeNoTemplate.h");
    // let file_path = Path::new(r"test\templateClass.h");
    let binding = get_file_path_from_user();
    let file_path = Path::new(&binding);
    let file_content = fs::read_to_string(file_path).expect("File does not Exist");

    let (template_class, template_type) = get_template_class(&file_content);
    let class_name = get_class_name(&file_content);
    let public_methods = get_public_methods(&file_content);

    if template_class.is_empty(){
        let (file_name, new_cpp_path) = get_file_name(file_path, "Imp.cpp".to_string());
        let converted_public_methods = convert_public_methods_to_cpp(public_methods, file_name, class_name);
        output_new_file(&new_cpp_path, &converted_public_methods);
    }
    else {
        let (file_name, new_cpp_path) = get_file_name(file_path, ".h".to_string());
        let converted_public_methods_template = convert_public_methods_to_cpp_template(public_methods, &template_class, class_name, &template_type);
        append_to_file(&new_cpp_path, &converted_public_methods_template);
    }

    println!("Done");
    let mut stop_console = String::new();
    std::io::stdin().read_line(&mut stop_console).expect("Failed to stop");
}

fn get_class_name(file_content: &String) -> String{
    let mut class_name = String::new();
    for line in file_content.lines(){
        let line = line.trim_start();

        if line.starts_with("//") || line.is_empty() || line.contains("#") || line.contains("};") || line.contains("virtual"){
            continue;
        }

        if line.starts_with("class"){
            class_name = line.replace("class", "").replace("{", "").trim().to_string();
        }
    }

    return class_name;
}

fn get_public_methods(file_content: &String) -> Vec<String>{
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

        if line.starts_with("private:") || line.starts_with("protected:") || line.starts_with("};"){
            is_public_methods = false;
            continue;
        }

        if is_public_methods{
            public_methods.push(line.to_string());
        }
    }

    return public_methods;
}

fn get_private_methods(file_content: &String) -> Vec<String>{
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

        if line.starts_with("public:") || line.starts_with("protected:")  || line.starts_with("};"){
            is_private_methods = false;
            continue;
        }

        if is_private_methods{
            private_methods.push(line.to_string());
        }
    }

    return private_methods;
}

fn get_template_class(file_content: &String)-> (String, String) {
    let mut template_name = String::new();
    let mut template_type = String::new();
    // let mut template_class: Vec<String> = vec![];
    // let mut is_template_class = false;
    for line in file_content.lines(){
        let line = line.trim_start();

        if line.starts_with("//") || line.is_empty() || line.contains("#") || line.contains("};") || line.contains("virtual"){
            continue;
        }

        if line.starts_with("template"){
            template_name = line.to_string();

            template_type = line.replace("template ", "").replace("class ", "");
            break;
        }
    }

    return (template_name, template_type);
}

fn get_file_path_from_user()-> String {
    let mut file_path = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        return file_path.to_string();
    }

    print!("Enter filePath: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to Read Line");
    
    let file_path = file_path.trim().to_string();

    return file_path;
}

fn convert_public_methods_to_cpp(public_methods: Vec<String>, file_name: String, class_name: String) -> String {

    let mut template = format!(
        "#include <iostream>\n#include \"{file_name}\"\n\nusing namespace std;\n\n")
        .to_string();

    for method in public_methods {
        let new_method = method.trim_end_matches(';');

        if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool"){

            let signature = new_method.splitn(2, " ").nth(1).unwrap();
            let new_method = format!("\n{} {}::{}{{\n\n}}\n", method.split_whitespace().next().unwrap(), class_name, signature);

            template.push_str(&new_method);
            println!("{}", new_method);
        }
        else {
            // for constructor/deconstructor
            let new_method = format!("\n{}::{}{{\n\n}}\n", class_name, new_method);

            template.push_str(&new_method);
            println!("{}", new_method);
        }
    }

    return template;
}

fn get_file_name(file_path: &Path, ext: String) -> (String, String){
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let new_cpp_name = Path::new(file_name).file_stem().unwrap().to_str().unwrap();
    let new_cpp_parent = file_path.parent().unwrap().to_str().unwrap();
    let new_cpp_path = format!("{}/{}{}", new_cpp_parent, new_cpp_name, ext);

    return (file_name.to_string(), new_cpp_path);
}

fn output_new_file(file_path: &String, converted_public_methods: &String){
    let mut new_cpp_file = fs::File::create(file_path).expect("Fail to create file");
    new_cpp_file.write(converted_public_methods.as_bytes()).expect("Fail to write to file");
}

fn convert_public_methods_to_cpp_template(public_methods: Vec<String>, template_class: &String, class_name: String, template_type: &String) -> String {

    let mut template = "\n".to_string();

    for method in public_methods {
        let new_method = method.trim_end_matches(';');

        let template_class = format!("\n{}", template_class);
        template.push_str(&template_class);
        if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool"){

            let signature = new_method.splitn(2, " ").nth(1).unwrap();
            let new_method = format!("\n{} {}{}::{}{{\n\n}}\n", method.split_whitespace().next().unwrap(), class_name, template_type, signature);

            template.push_str(&new_method);
            println!("{}", new_method);
        }
        else {
            // for constructor/deconstructor
            let new_method = format!("\n{}{}::{}{{\n\n}}\n", class_name, template_type, new_method);

            template.push_str(&new_method);
            println!("{}", new_method);
        }
    }

    return template;
}

fn append_to_file(file_path: &String, converted_public_methods_template: &String){
    let mut new_cpp_file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to Open File");

    new_cpp_file.write(converted_public_methods_template.as_bytes()).expect("Fail to write");
}