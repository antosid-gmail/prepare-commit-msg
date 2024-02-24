use git2::{Error as GitError, Repository};
use std::{env, fs, process};
use std::io::Error as IoError;

fn get_branch_name() -> Result<String, GitError> {
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    Ok(head.shorthand().unwrap_or("").to_string())
}

fn update_commit_message(commit_msg_file: &str, branch_name: &str) -> Result<(), IoError> {
    let commit_msg = fs::read_to_string(commit_msg_file)?;
    let new_commit_msg = format!("{}: {}", branch_name, commit_msg);
    fs::write(commit_msg_file, new_commit_msg)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() < 2 {
        eprintln!("Usage: {} <commit_msg_file>", args[0]);
        process::exit(1);
    }

    let commit_msg_file = &args[1];

    let branch_name = get_branch_name().unwrap();

    if !branch_name.is_empty() && branch_name != "master" && branch_name != "main" {
        update_commit_message(commit_msg_file, &branch_name).unwrap();
    }
}
