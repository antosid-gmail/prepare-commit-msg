use git2::{Error as GitError, Repository};
use std::fs;
use std::io::Error as IoError;

pub fn get_branch_name() -> Result<String, GitError> {
    let repo = Repository::open(".").map_err(|e| GitError::from_str(&format!("Failed to open repository: {}", e)))?;
    let head = repo.head().map_err(|e| GitError::from_str(&format!("Failed to get head: {}", e)))?;
    head.shorthand().map(|s| s.to_string()).ok_or_else(|| GitError::from_str("No branch name found"))
}

pub fn update_commit_message(commit_msg_file: &str, branch_name: &str) -> Result<(), IoError> {
    let commit_msg = fs::read_to_string(commit_msg_file)?;
    let new_commit_msg = format!("{}: {}", branch_name, commit_msg);
    fs::write(commit_msg_file, new_commit_msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_branch_name() {
        let branch_name = get_branch_name().expect("Failed to get branch name");
        assert!(!branch_name.is_empty());
    }

    #[test]
    fn test_update_commit_message() {
        let commit_msg_file = "test_commit_msg.txt";
        let branch_name = "test-branch";
        let commit_msg = "Test commit message";
        fs::write(commit_msg_file, commit_msg).unwrap();
        update_commit_message(commit_msg_file, branch_name).unwrap();
        let new_commit_msg = fs::read_to_string(commit_msg_file).unwrap();
        assert_eq!(new_commit_msg, format!("{}: {}", branch_name, commit_msg));
        fs::remove_file(commit_msg_file).unwrap();
    }
}