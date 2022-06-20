// A specific Module to store all the functions related / used in git 

pub mod mdm_git{
    use git2::Repository;
    use std::path::Path;
/*fn main() {
    let repo_path: PathBuf = "repo".parse().unwrap();
    let file_name = "some-file";
    let repo = initialise(&repo_path);
    create_initial_commit(&repo);
    create_file(&repo_path, file_name);
    add_all(&repo);
    commit(&repo);
}*/
#[allow(dead_code)]

fn initialise(repo_path: &Path) -> git2::Repository {
    std::fs::create_dir(&repo_path).unwrap();
    git2::Repository::init(&repo_path).unwrap()
}
#[allow(dead_code)]

fn create_initial_commit(repo: &git2::Repository) {
    let signature = repo.signature().unwrap();
    let oid = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )
    .unwrap();
}
#[allow(dead_code)]
fn create_file(repo_path: &Path, file_name: &str) {
    let filepath = repo_path.join(file_name);
    std::fs::File::create(filepath).unwrap();
}

pub fn add_all(repo: &git2::Repository) {
    repo.index()
        .unwrap()
        .add_all(&["."], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
}

pub fn commit(repo: &git2::Repository) {
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = repo.signature().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    index.write().expect("i have no idea");
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Added some data",
        &tree,
        &[&parent_commit],
    )
    .unwrap();
}
use git2::Direction;

pub fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/main:refs/heads/main"], None)
}
}