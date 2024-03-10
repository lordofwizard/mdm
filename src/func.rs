use std::{env, fs::File, io::{ErrorKind, Error, Write}};
use chrono::Local;
use git2::{Cred, PushOptions, RemoteCallbacks, Repository};

use git2::{StatusOptions};
use std::fs;
use std::fs::OpenOptions;


pub fn run(){
    // This function is the main trigger of the whole program
    create_cache_folder();
    let message : String = get_args_as_string();
    match write_or_create_file(format!("{}/{}.txt",cache_folder_path().unwrap(),file_name()), message){
        Err(err) => eprintln!("Error: {}", err),
        _ => {}
    }
    
    match check_and_commit_to_git_repo(cache_folder_path().unwrap().as_str()) {
        Ok(()) => {},
        _ => {}
    }
}

fn get_args_as_string() -> String {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Join the arguments into a single string, excluding the first argument (program name)
    let joined_args: String = args.iter().skip(1).cloned().collect::<Vec<String>>().join(" ");

    // Return the joined arguments
    joined_args
}

fn file_name() -> String {
    // This function returns a String that contains the file_name we are gonna use
    // for example the file name is generally in this format
    // year-month-day+time_in_the_world.txt
    // For me, its kolkatta i.e. 5.30 + from global time.
    
    let time_stamp: String = Local::today().to_string();
    time_stamp
}


fn check_and_commit_to_git_repo(path: &str) -> Result<(), git2::Error> {
    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(_) => {
            let path = cache_folder_path().unwrap();
            let repo = match Repository::init(path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to init: {}", e),
            };
            println!("REPOSITORY WASN\"T PRESENT, RECOMMENDED TO FOLLOW THE GUIDE AND SETUP PRPOERLY");
            repo
        }
        
    };

    // Check if there are any changes
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut opts))?;

    if !statuses.is_empty() {
        println!("Changes detected in the repository at {}", path);
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;

        // Commit changes
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let sig = repo.signature()?;
        let head = repo.head()?;
        let parent_commit = repo.find_commit(head.target().unwrap())?;
        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Committed changes",
            &tree,
            &[&parent_commit]
        )?;

        // Push changes to remote origin
        let mut remote = repo.find_remote("origin")?;
        match remote.push(&["refs/heads/main:refs/heads/main"], None){
            Ok(()) => {},
            Err(E) =>eprintln!("Error : {}",E)
        };
        println!("Changes committed and pushed to remote origin.");
    } else {
        println!("No changes detected in the repository at {}", path);
    }

    Ok(())
}



fn write_or_create_file(path: String, data: String) -> Result<(), Error> {
    // Open the file in write mode, creating it if it doesn't exist
    let mut  present = false;
    let mut file = match OpenOptions::new().create(true).append(true).open(&path){
        Ok(file) => {
            present = true;
            file
        }
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            File::create(&path)?
        }
        Err(err) => return Err(err),
    };
    if present == true {
        
    }
    // Write the data into the file
    file.write_all(data.as_bytes())?;

    // Flush the file to ensure all data is written
    file.flush()?;

    Ok(())
}

fn cache_folder_path() -> Option<String>{
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => {
            println!("Unable to determine home directory.");
            return Option::None
        }
    };
    return Some(format!("{}/.cache/mdm", home_dir))
}

fn create_cache_folder() {
    // Get the home directory

    let cache_folder_path: String = cache_folder_path().unwrap();

    if fs::metadata(&cache_folder_path).is_ok() {
        //println!("Folder already present: {}", cache_folder_path);
    } else {
        if let Err(err) = fs::create_dir_all(&cache_folder_path) {
            println!("Error creating cache folder: {}", err);
        } else {
            println!("Cache folder created: {}", cache_folder_path);
        }
    }
}

