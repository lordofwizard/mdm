use std::io;
use std::fs::*;
use colored::*;
use git2::Repository;
use git2::Branch;
use git2::BranchType::*;

fn branch_dena(&mut repo: Repository) -> git2::Branch{
    let br = match &mut repo.find_branch("master",BranchType::Local) {
        Ok(br ) => br,
        Err(e) => panic!("Some error {}",e)
    };
}

fn main() {
    let repo = match Repository::open("./data/") {
        Ok(repo) => repo,
        Err(e) => panic!("{} {}","Something went wrong reading the repository \n".red(),e)
    };
    let branch = match Branch::rename(branch_dena(&mut),"main",true){

    }

    println!("Hello, Gaurav!");
}
