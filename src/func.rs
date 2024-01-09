use std::env;
use chrono::Local;
use std::fs;

pub fn run(){
    // This function is the main trigger of the whole program
    create_cache_folder();
}

fn args_parser(){
    // Parsing which type of program mode is this, a cli string or editor type
    enum TypeOfArgs {
        CommandLine,
        Editor,
        MultiArg
    }
    
    let arg_type : TypeOfArgs;
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        arg_type = TypeOfArgs::Editor;
    }
    else if args.len() == 2 {
        arg_type = TypeOfArgs::CommandLine;
    }
    else {
        arg_type = TypeOfArgs::MultiArg;
    }
}

fn file_name() -> String {
    // This function returns a String that contains the file_name we are gonna use
    // for example the file name is generally in this format
    // year-month-day+time_in_the_world.txt
    // For me, its kolkatta i.e. 5.30 + from global time.
    
    let time_stamp = Local::today().to_string();
    time_stamp
}


fn create_cache_folder() {
    // Get the home directory
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => {
            println!("Unable to determine home directory.");
            return;
        }
    };
    let cache_folder_path = format!("{}/.cache/mdm", home_dir);

    if fs::metadata(&cache_folder_path).is_ok() {
        println!("Folder already present: {}", cache_folder_path);
    } else {
        if let Err(err) = fs::create_dir_all(&cache_folder_path) {
            println!("Error creating cache folder: {}", err);
        } else {
            println!("Cache folder created: {}", cache_folder_path);
        }
    }
}

