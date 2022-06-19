
// File to store all the basic functionality 
pub mod mdm{
    use colored::*;
    use git2::BranchType;
    use git2::Repository;

    pub fn run(){
        let repo = match Repository::open("./data/") {
            Ok(repo) => repo,
            Err(e) => panic!("{} {}","Something went wrong reading the repository \n".red(),e)
        };
        let br= &mut repo.find_branch("master",BranchType::Local).expect("Something went wrong");
        br.rename("main",true).expect("Something went wrong while remaning the branch");
    }

    
    fn directory_maker(){
        
    }
    fn database_maker(){

    }   
    fn data_pusher(){

    }
    use chrono::*;
    pub fn date_printer() -> String{
        let local= Local::today().to_string();
        local
    }
    pub fn making_file(){
        use std::fs::File;
        let today = date_printer();
        let today_str = today.as_str();
        let path = "./data/".to_string() + &today + ".txt" ;
        let mut file = File::create(path.as_str());
    }
}