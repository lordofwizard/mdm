
// File to store all the basic functionality 
pub mod mdm{
    use colored::*;
    use git2::BranchType;
    use git2::Repository;
    use std::fs::File;
    use std::path::Path;

    pub fn run(){
        let repo = match Repository::open("./data/") {
            Ok(repo) => repo,
            Err(e) => panic!("{} {}","Something went wrong reading the repository \n".red(),e)
        };
        let mut main_br = repo.find_branch("main",BranchType::Local).unwrap();
        let mut br= match repo.find_branch("master",BranchType::Local){
            Ok(br) => br,
            Err(e) => main_br
        };

        br.rename("main",true).unwrap();
        making_file();
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
        

    }

    use chrono::*;
    fn date_printer() -> String{
        let local= Local::today().to_string();
        local
    }
    fn making_file(){
        let today = date_printer();
        let path = "./data/".to_string() + &today + ".txt";
        let variant = matches!(can_make_file(),CanMakeFile::Yes);
        if variant == true{
            let mut file = File::create(path.as_str()).expect("idk");
        }
        else{
            println!("{}","File Already exists or something wrong with the path".red());
        }
    }
}