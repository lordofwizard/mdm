use colored::*;
use git2::BranchType;
use git2::Repository;



/*fn branch_dena(&mut repo : &mut git2::Repository) -> &mut Branch<'_>{
    let mut br = match &mut repo.find_branch("master",BranchType::Local) {
        Ok(br ) => br,
        Err(e) => panic!("Some error {}",e)
    };
    &mut br
}*/

fn main() {
    let repo = match Repository::open("./data/") {
        Ok(repo) => repo,
        Err(e) => panic!("{} {}","Something went wrong reading the repository \n".red(),e)
    };
    let br= &mut repo.find_branch("master",BranchType::Local).expect("Something went wrong");
    br.rename("main",true).expect("Something went wrong while remaning the branch");
}

