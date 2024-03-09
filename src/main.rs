use prepare_commit_msg::{get_branch_name, update_commit_message};
use std::{env, process};

const BRANCH_NAMES: [&str; 2] = ["main", "master"];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <commit_msg_file>", args[0]);
        process::exit(1);
    }

    let commit_msg_file = &args[1];

    let branch_name = match get_branch_name() {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Failed to get branch name: {}", err);
            return;
        }
    };

    if branch_name.is_empty() || BRANCH_NAMES.contains(&branch_name.as_str()) {
        println!("No branch name found or on default branch. Skipping commit message update.");
        return;
    }

    match update_commit_message(commit_msg_file, &branch_name) {
        Ok(()) => println!("Commit message updated with branch name."),
        Err(err) => eprintln!("Failed to update commit message: {}", err),
    }
}
