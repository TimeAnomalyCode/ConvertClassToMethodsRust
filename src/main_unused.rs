// use std::fs::OpenOptions;
// use std::io;
// use std::fs;
// use std::io::Write;
// use std::path::Path;

// fn main() {

//     let mut file_path = String::new();

//     print!("Enter filePath: ");
//     let _ = io::stdout().flush();
//     io::stdin()
//         .read_line(&mut file_path)
//         .expect("Failed to Read Line");
    
//     let file_path = Path::new(file_path.trim());

//     let content = fs::read_to_string(file_path)
//         .expect("Failed To Read File");

//     if !content.contains("template"){
//         let file_name = file_path.file_name().unwrap().to_str().unwrap();
//         let new_cpp_name = Path::new(file_name).file_stem().unwrap().to_str().unwrap();
//         let new_cpp_path = file_path.parent().unwrap().to_str().unwrap();
//         let new_cpp = format!("{}/{}.cpp", new_cpp_path, new_cpp_name);

//         let mut new_cpp_file = fs::File::create(new_cpp)
//             .expect("Failed to create file");

//         let (public_methods, class_variables, class_name, template_class) = get_public_methods_and_class_variables_and_class_name_and_template(content);

//         let mut template = format!(
//             "#include <iostream>\n#include \"{file_name}\"\n\nusing namespace std;\n\n")
//             .to_string();

//         let cpp_public_methods = change_public_methods_to_cpp(public_methods, class_name);
//         template.push_str(&cpp_public_methods);
//         // for method in public_methods{

//         //     let new_method = method.trim_end_matches(';');

//         //     if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool"){

//         //         let signature = new_method.splitn(2, " ").nth(1).unwrap();
//         //         let new_method = format!("\n{} {}::{}{{\n\n}}\n", method.split_whitespace().next().unwrap(), class_name, signature);

//         //         template.push_str(&new_method);
//         //         println!("{}", new_method);
//         //     }
//         //     else {
//         //         // for constructor/deconstructor
//         //         let new_method = format!("\n{}::{}{{\n\n}}\n", class_name, new_method);

//         //         template.push_str(&new_method);
//         //         println!("{}", new_method);
//         //     }
//         // }

//         new_cpp_file.write(template.as_bytes()).expect("Write Failed");
//     }
//     else {
//         let mut new_cpp = OpenOptions::new()
//             .append(true)
//             .open(file_path)
//             .expect("Failed to Open File");

//         let (public_methods, class_variables, class_name, template_class) = get_public_methods_and_class_variables_and_class_name_and_template(content);

//         let template = change_public_methods_to_cpp_template(public_methods, class_name, template_class);
    
//         new_cpp.write(template.as_bytes()).expect("Failed to Write");
//     }
//     // println!("{content}");
// }

// fn get_public_methods_and_class_variables_and_class_name_and_template(content: String) -> (Vec<String>, Vec<String>, String, String) {
//     let mut class_name = String::new();
//     let mut template_class = String::new();

//     let mut is_public_method = false;
//     let mut public_methods: Vec<String> = vec![];
    
//     let mut is_class_variable = false;
//     let mut class_variables: Vec<String> = vec![];

//     for line in content.lines(){
//         let new_line = line.trim_start();
//         if new_line.starts_with("//") || new_line.is_empty() || new_line.contains("#") || new_line.contains("}") || new_line.contains("virtual"){
//             continue;
//         }

//         if new_line.starts_with("class"){
//             class_name = new_line.replace("class", "").trim().to_owned();
//         }

//         if new_line.starts_with("template"){
//             template_class = new_line.to_owned();
//         }

//         if new_line.starts_with("public:"){
//             is_public_method = true;
//             continue;
//         }

//         if is_public_method {
//             if new_line.contains("private:") || new_line.contains("protected:") || new_line.contains("}"){
//                 is_public_method = false;
//             }
//             else {
//                 public_methods.push(new_line.to_string());
//             }
//         }

//         if new_line.contains("protected:") || new_line.contains("private:"){
//             is_class_variable = true;
//             continue;
//         }

//         if is_class_variable {
//             if new_line.contains("public:") || new_line.contains("}"){
//                 is_class_variable = false;
//             }
//             else {
//                 class_variables.push(new_line.to_string());
//             }
//         }
//         // println!("{}", new_line);
//     }

//     return (public_methods, class_variables, class_name, template_class);
// }

// fn change_public_methods_to_cpp(public_methods: Vec<String>, class_name: String) -> String{
//     let mut template = String::new();
//     for method in public_methods{

//         let new_method = method.trim_end_matches(';');

//         if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool"){

//             let signature = new_method.splitn(2, " ").nth(1).unwrap();
//             let new_method = format!("\n{} {}::{}{{\n\n}}\n", method.split_whitespace().next().unwrap(), class_name, signature);

//             template.push_str(&new_method);
//             println!("{}", new_method);
//         }
//         else {
//             // for constructor/deconstructor
//             let new_method = format!("\n{}::{}{{\n\n}}\n", class_name, new_method);

//             template.push_str(&new_method);
//             println!("{}", new_method);
//         }
//     }

//     return template;
// }

// fn change_public_methods_to_cpp_template(public_methods: Vec<String>, class_name: String, template_class: String) -> String{
//     let mut template = String::new();
//     let template_class = format!("\n{}\n", template_class);
//     for method in public_methods{

//         let new_method = method.trim_end_matches(';');
//         template.push_str(&template_class);

//         if method.starts_with("void") || method.starts_with("int") || method.starts_with("char") || method.starts_with("bool") || method.starts_with("T"){

//             let signature = new_method.splitn(2, " ").nth(1).unwrap();
//             let new_method = format!("\n{} {}<T>::{}{{\n\n}}\n", method.split_whitespace().next().unwrap(), class_name, signature);

//             template.push_str(&new_method);
//             println!("{}", new_method);
//         }
//         else {
//             // for constructor/deconstructor
//             let new_method = format!("\n{}<T>::{}{{\n\n}}\n", class_name, new_method);

//             template.push_str(&new_method);
//             println!("{}", new_method);
//         }
//     }

//     return template;
// }
