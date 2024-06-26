use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;
// use std::process::Command;

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(), // Return an empty string if the input is empty
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn rm_dir(dir_path: &str) {
    match fs::remove_dir_all(dir_path) {
        Ok(_) => println!(""),
        Err(e) => println!("Error deleting directory: {}", e),
    }
}
#[derive(Debug, Deserialize, Default)]
pub struct MvcStruct {
    pub module_name: String,
    pub crud: bool,
}
pub async fn read_file(file_path: &str) -> Vec<MvcStruct> {
    if !Path::new(file_path).exists() {
        println!("The Config File Doesn't Exit, Provide A Valid Path");
        panic!("")
    }

    if let Ok(file_txt) = fs::read_to_string(&file_path) {
        if let Ok(file) = serde_json::from_str::<Vec<MvcStruct>>(&file_txt) {
            return file;
        }
    }
    panic!("unable to read file {} ", file_path);
}

pub fn prompt_user() -> String {
    // Prompt the user for input
    println!("Type [yes|no] : ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Read line failed.");

    let mut input = String::from(input.trim().to_lowercase());

    if input != "yes" && input != "no" {
        input = prompt_user()
    }

    return input;
}

// pub async fn execute(commande: String) -> Result<String, bool> {
//     match Command::new("sh").arg("-c").arg(&commande).output() {
//         Ok(output) => {
//             if !output.status.success() {
//                 return Err(false);
//             }
//             Ok(String::from(format!("executed {:?} ", &commande)))
//         }
//         Err(err) => panic!(" {} ", err),
//     }
// }

pub fn rust_ascii() -> String {
    return "                                          #(    #(,    (*                                    
                                     ((   ((((#*(((((,((((,  /((                              
                               (/   #((((/((((((((((((((((((((((    #                         
                           ((((((((((((((((((((((((((((((((((((((/                 *      
        (#            ((((((((((((((((((((((((((((((((((((((((((((#(((#        ,(((,    ( 
   ((   #(((#         ,(((((((((((((((((((((((((((((((((((((((((((((((        (((((    (((
  ((((   (((((    ((((((((((((((((((((((((((((((((((((((((((((((((((((((((#  ((((((  (((( 
  (((((( (((((     (((((((((((((((((((((((((((((((((((((((((((((((((((((((   ,((((((((((  
    (((((((((*   /((((((((((((((((((((((((((((((((((((((((((((((((((((((((#,   (((((((    
        ((((   *((((((((((((((((((((((/  @@@((((((((  @@@/(((((((((((((((((((  (((*       
          ((((#  (((((((((((((((((((((/  /@@@/((((/@   @@@(((((((((((((((((##((/,         
             ((((((((((((((((((((((((/@@@@@@@/((((/@@@@@@@((((((((((((((((((((#           
            #((((((((((((((((((((((((((/@@@%((((((((%@@%/(((((((((((((((((((((((((        
           *((((( .%# &%%#(((((((((((((((((((&@@@@@((((((((((((((((#%%&  &%# ((((         
              ((((  (((      (&#%%%#(((((((((((((((((((((#%%#%%&,       &%  ))))          
                #((/  ((                                               #   )))            
                   ((   (                                             )   ))              
                    ((   (                                           )   ))
    "
    .to_string();
}
