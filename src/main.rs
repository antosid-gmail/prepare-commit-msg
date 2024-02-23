use std::env;
use std::fs;
use git2::{Repository, Error};

/// This is the main function of the program.
/// It takes a commit message file as a command line argument and appends the branch name to the commit message.
/// The branch name is only appended if it is not empty and not equal to "master" or "main".
/// The modified commit message is then written back to the commit message file.
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() < 2 {
        eprintln!("Usage: {} <commit_msg_file>", args[0]);
        std::process::exit(1);
    }

    let commit_msg_file = &args[1];

    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("");

    if !branch_name.is_empty() && branch_name != "master" && branch_name != "main" {
        let commit_msg = fs::read_to_string(commit_msg_file)
            .map_err(|e| Error::from_str(&format!("Failed to read file: {}", e)))?;

        let new_commit_msg = format!("{}: {}", branch_name, commit_msg);

        fs::write(commit_msg_file, new_commit_msg)
            .map_err(|e| Error::from_str(&format!("Failed to write to file: {}", e)))?;
    }

    Ok(())
}
