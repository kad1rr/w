use std::process::Command;
use lazy_static::lazy_static;

lazy_static! {
    static ref IS_GIT_INSTALLED: bool = {
        match Command::new("git").arg("--version").output() {
            Ok(result) => result.status.success(),
            Err(_) => false,
        }
    };
}

pub fn is_git_installed() -> bool {
    *IS_GIT_INSTALLED
}

pub fn get_branch() -> Result<String, &'static str> {
    if !is_git_installed() {
        return Err("Git not installed");
    }

    match Command::new("git").args(&["branch", "--show-current"]).output() {
        Ok(output) => {
            if output.status.success() {
                let branch_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(branch_name)
            } else {
                Err("Failed to get Git branch")
            }
        }
        Err(_) => Err("Failed to execute Git command"),
    }
}