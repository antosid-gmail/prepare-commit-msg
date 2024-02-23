use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let commit_msg_file = &args[1];

    let output = Command::new("git")
        .arg("symbolic-ref")
        .arg("--short")
        .arg("HEAD")
        .output()
        .expect("failed to execute git command");

    let branch_name = String::from_utf8_lossy(&output.stdout);
    let branch_name = branch_name.trim().replace("/", "_");

    if !branch_name.is_empty() && branch_name != "master" && branch_name != "main" {
        let commit_msg =
            fs::read_to_string(commit_msg_file).expect("failed to read commit message file");

        let new_commit_msg = format!("{}: {}", branch_name, commit_msg);

        fs::write(commit_msg_file, new_commit_msg).expect("failed to write to commit message file");
    }
}
