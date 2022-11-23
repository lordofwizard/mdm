
// File to store all the basic functionality
pub mod mdm{

    use colored::*;
    use git2::BranchType;
    use git2::Repository;
    use std::fs::File;
    use std::path::Path; pub use crate::git::mdm_git; use crate::git::mdm_git::add_all;
    use crate::git::mdm_git::commit;

    pub fn run(){
        let repo = match Repository::open("./data/") {
            Ok(repo) => repo,
            Err(e) => panic!("{} {}","Something went wrong reading the repository \n".red(),e)
        };


        let mut br= match repo.find_branch("master",BranchType::Local){
            Ok(br) => br,
            Err(_e) => {
                let main_br = repo.find_branch("main",BranchType::Local).expect("main branch not found");
                main_br
            }
        };
        // functionality that checks if the file has main branch if not then this given line
        // renames branch with it.
        br.rename("main",true).unwrap();

        // Call to making_file() which will check if the file exists or not.
        making_file();
        use std::io::Write;
        let mut file = making_file();
        let dat = data();
        println!("{}",&dat);
        file.write_all(dat.as_bytes()).expect("Unable to put data inside the file");
        add_all(&repo);
        commit(&repo);
        //mdm_git::push(&repo,"git@github.com:lordofwizard/temp_data.git").expect("counldn't push bitch");
        use std::process::Command;
        Command::new("git")
        .current_dir("./data").args(["push","--quiet","-u","--no-progress","origin","main"])
        .spawn()
        .expect("ls command failed to start");
        println!("{}", "Succesfully Uploaded to github".green());
    }

    enum CanMakeFile{
        Yes,
        No
    }

    fn can_make_file()-> CanMakeFile{
        let today = date_printer();
        let path = "./data/".to_string() + &today + ".txt";
        let path = Path::new(&path);
        if std::path::Path::exists(path) == true{
            println!("{}", "Hurray file already exists".blue());
            CanMakeFile::No
        }
        else{
            let path = "./data/".to_string() + &today + ".txt";

            let result : CanMakeFile = match File::create(path.as_str()){
                Ok(_file) => CanMakeFile::Yes,
                Err(_e) => CanMakeFile::No,
            };
            result
        }

//idk some change
    }

    use chrono::*;
    // This function returns a own string which contains  Date in standard format.
    fn date_printer() -> String{
        let local= Local::today().to_string();
        local
    }
    fn making_file()-> File{
        use std::fs::OpenOptions;
        let today = date_printer();
        let path = "./data/".to_string() + &today + ".txt";
        let variant = matches!(can_make_file(),CanMakeFile::Yes);
        if variant == true{
            let file = File::create(path.as_str()).expect("Error while CREATING the file");
            file
        }
        else if std::path::Path::new(path.as_str()).exists() == true {
            println!("File is already present good that's nice");
            OpenOptions::new().append(true).open(path.as_str()).expect("Error opening the file bitch")
        }
        else {
            panic!("something went wrong while making the file");
        }


    }
    // Takes user input and then this function gives a Owned String as output.
    fn user_input()-> String{
        let pattern = std::env::args().nth(1).expect("Please input some kind of message to upload inside the file");
        pattern
    }

    // This method takes nothing but converts a string to it's part with +++ and --- at start.
    fn data() -> String{
        let data : String = "+++++++++++++++++++++++++++++++++++++++++\n".to_owned() + &user_input() + "\n-------------------------------------------\n";
        data
    }
}
