//
// Entirely borrowed from https://gitlab.com/ogarcia/lazycc/-/blob/master/src/hook.rs
// SPDX: GPL-3.0-only
//
use {
    anyhow::{Context, Error, Result},
    std::{
        path::PathBuf,
        process::{Command, Output},
    },
};

pub fn get_absolute_path() -> Result<PathBuf> {
    let Output {
        status,
        stdout,
        stderr,
    } = Command::new("git")
        .args(["rev-parse", "--absolute-git-dir"])
        .output()
        .context("Failed run git rev-parse command")?;

    if !status.success() {
        let mut stderr = String::from_utf8(stderr).unwrap();
        stderr.pop();
        let err = Error::msg(format!("{}", stderr)).context(format!(
            "Failed to find hooks dir, git rev-parse failed with code {}",
            status.code().unwrap()
        ));
        return Err(err);
    }

    let mut path = String::from_utf8(stdout)?;
    path.pop();
    Ok(PathBuf::from(path))
}
