use std::process::Command;
use std::env;
use std::fs;

pub fn clone(repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let local_repo = format!("{home}/.packit/repo/{}", repo.split("/").last().unwrap_or("default"));

    fs::remove_dir_all(&local_repo)?;
    fs::create_dir_all(&local_repo)?;

    let child = Command::new("git")
        .args(["clone", repo, &local_repo, "--progress"])
        .output()?;

    if child.status.success() {
        Ok(local_repo)
    } else {
        Err("exited with non-zero exit code".into())
    }
}


