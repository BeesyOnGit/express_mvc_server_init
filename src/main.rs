mod utils;

use serde::Deserialize;
use std::env::consts::OS;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
// use std::process::Command;
use utils::content_utils::{
    app_content, controllers_content, db_connect_content, middleware_content, model_content,
    package_json_content, router_content,
};
use utils::utils::{capitalize, execute, prompt_user, read_file, rm_dir, rust_ascii};

#[tokio::main]
async fn main() {
    let mut file_path = String::from("");
    let mut work_dir = String::from("");

    let files_struct: [FileStructure; 7] = [
        FileStructure::new("Controllers", "folder"),
        FileStructure::new("Models", "folder"),
        FileStructure::new("Routes", "folder"),
        FileStructure::new("Middleware", "folder"),
        FileStructure::new("App.ts", "file"),
        FileStructure::new("DBConnection.ts", "file"),
        FileStructure::new("package.json", "file"),
    ];

    let args: Vec<String> = std::env::args().skip(1).collect();

    for i in 0..args.len() {
        if args[i] == "--path" && i + 1 < args.len() {
            file_path = String::from(&args[i + 1]);
        }

        if args[i] == "--workdir" && i + 1 < args.len() {
            work_dir = String::from(&args[i + 1]);
        }
    }

    if file_path.is_empty() || work_dir.is_empty() {
        println!("The --path or --workdir arguments are missing please provide them !");
        return;
    }

    let configs = read_file(&file_path).await;

    println!(
        "Would You Like To Start From Scratch (remove all Controllers & Routes folder content)"
    );

    let reset = prompt_user();

    // if users choses to, reset all the files
    if reset == "yes" {
        for file in &files_struct {
            if !Path::new(&format!("{}/App/{}", work_dir, file.name)).exists() {
                continue;
            }

            if file.element_type == "folder" {
                // remove directory
                rm_dir(&format!("{}/App/{}", work_dir, file.name));
                continue;
            }

            fs::remove_file(&format!("{}/App/{}", work_dir, file.name))
                .expect("error while deleting file");
        }
    }

    // re create all the files
    for file in &files_struct {
        if Path::new(&format!("{}/App/{}", work_dir, file.name)).exists() {
            continue;
        }

        if file.element_type == "folder" {
            // create directory
            fs::create_dir_all(&format!("{}/App/{}", work_dir, file.name))
                .expect("error while creating file");

            continue;
        }

        File::create(&format!("{}/App/{}", work_dir, file.name))
            .expect("error while creating file");
    }

    // files strucutre creation
    for structure in files_struct.iter() {
        for config in configs.iter() {
            let curr_name = capitalize(&config.module_name);

            if structure.name == "Middleware" {
                let file_exist = Path::new(&format!(
                    "{}/App/{}/ServerFunctions.ts",
                    work_dir, structure.name,
                ))
                .exists();

                if file_exist {
                    continue;
                }

                let mut current_file = File::create(&format!(
                    "{}/App/{}/ServerFunctions.ts",
                    work_dir, structure.name,
                ))
                .expect("Error creating file");

                let file_content = middleware_content();
                current_file
                    .write_all(file_content.as_bytes())
                    .expect("error writing the file");
                continue;
            }

            if structure.element_type == "file" {
                let mut current_file =
                    File::create(&format!("{}/App/{}", work_dir, structure.name))
                        .expect("Error creating file");

                let file_content = execute_function(&structure.name, &curr_name, &config.crud);
                current_file
                    .write_all(file_content.as_bytes())
                    .expect("error writing the file");

                continue;
            }

            let file_exist = Path::new(&format!(
                "{}/App/{}/{}{}.ts",
                work_dir, structure.name, curr_name, structure.name
            ))
            .exists();

            if file_exist {
                continue;
            }

            let mut current_file = File::create(&format!(
                "{}/App/{}/{}{}.ts",
                work_dir, structure.name, curr_name, structure.name
            ))
            .expect("Error creating file");

            let file_content = execute_function(&structure.name, &curr_name, &config.crud);
            current_file
                .write_all(file_content.as_bytes())
                .expect("error writing the file");
        }
    }

    let mut app_file = String::new();
    if let Ok(fi) = fs::read_to_string(&format!("{}/App/App.ts", work_dir)) {
        app_file = fi
    }

    if app_file.is_empty() {
        println!("App.ts is empty");
        return;
    }

    let split = String::from("//split ^");
    let mut app_file_splited: Vec<String> = app_file.split(&split).map(|s| s.to_owned()).collect();

    // let mut combined_string = String::new();
    for conf in configs.iter() {
        let name = conf.module_name.clone();

        //add the imports
        let import_str = String::from(format!(
            "\n    import {}Routes from './Routes/{}Routes'",
            &capitalize(&name),
            &capitalize(&name)
        ));
        app_file_splited[0].push_str(&import_str);

        //add the routes
        let route_str = String::from(format!(
            "\n    app.use('/{}', {}Routes);",
            &name,
            &capitalize(&name)
        ));
        app_file_splited[1].push_str(&route_str);
    }

    app_file = app_file_splited.concat();

    let mut current_file =
        File::create(&format!("{}/App/App.ts", work_dir)).expect("Error creating file");

    current_file
        .write_all(app_file.as_bytes())
        .expect("error writing the file");

    // let _ex = execute(format!("cd {} && npm i", work_dir)).await;
    // let _ = match OS {
    //     "linux" => execute(format!("cd {} && npm i", work_dir)).await,
    //     "macos" => execute(format!("cd {} && npm i", work_dir)).await,
    //     "windows" => execute(format!("cd {} ", work_dir)).await,
    //     _ => execute(format!("cd {} ", work_dir)).await,
    // };

    println!("\n \n \n \n \n \n");
    print!("{}", rust_ascii());
    println!("\n");
    println!("Task Done Check {} to begin work!", work_dir);
    println!("PS: Don't forget to install dependencies if they are not");
    println!("\n");
    println!("program writen in Rust 🦀 with ❤");
}

fn execute_function(func_type: &str, param1: &str, param2: &bool) -> String {
    match func_type {
        "Controllers" => return controllers_content(param1, param2),
        "Routes" => return router_content(param1, param2),
        "Models" => return model_content(param1),
        "App.ts" => return app_content(),
        "package.json" => return package_json_content(),
        "DBConnection.ts" => return db_connect_content(),
        _ => return "".to_string(),
    };
}

#[derive(Debug, Deserialize, Default)]
struct FileStructure {
    name: String,
    element_type: String,
}
impl FileStructure {
    fn new(name: &str, typ: &str) -> FileStructure {
        FileStructure {
            name: String::from(name),
            element_type: String::from(typ),
        }
    }
}
