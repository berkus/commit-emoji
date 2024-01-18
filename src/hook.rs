//
// Entirely borrowed from https://gitlab.com/ogarcia/lazycc/-/blob/master/src/hook.rs
// SPDX: GPL-3.0-only
//
use {
    super::repo,
    anyhow::{Context, Error, Result},
    std::{
        fs::{read_to_string, remove_file, set_permissions, File, Permissions},
        io::Write,
        os::unix::prelude::PermissionsExt,
        path::PathBuf,
    },
};

const PERMISSIONS: u32 = 0o775;
const FILENAME: &str = "commit-msg";
const CONTENTS: &str = "#!/bin/sh\ncommit-emoji \"${@}\"\n";

fn get_hook_absolute_path() -> Result<PathBuf> {
    let mut path = repo::get_absolute_path()?;
    path.push("hooks");
    path.push(FILENAME);
    Ok(path)
}

fn check_installed() -> Result<PathBuf> {
    let hook_path = get_hook_absolute_path()?;

    if hook_path.exists() {
        let existing_hook = read_to_string(&hook_path)
            .context(format!("Failed to read hook file {:?}", &hook_path))?;

        let e = match existing_hook == CONTENTS {
            true => Error::msg("Hook is already installed! Just run \"git commit\" normaly"),
            false => Error::msg(format!("There is already another hook \"{}\"", FILENAME)),
        };

        return Err(e);
    }

    Ok(hook_path)
}

pub fn install() -> Result<()> {
    let hook_path = check_installed()?;

    let mut file =
        File::create(&hook_path).context(format!("Failed to create hook {:?}", &hook_path))?;
    file.write_all(CONTENTS.as_bytes())
        .context(format!("Failed to write hook contents in {:?}", &hook_path))?;

    set_permissions(hook_path, Permissions::from_mode(PERMISSIONS))?;

    println!("Hook installed for this repo!");

    Ok(())
}

pub fn uninstall() -> Result<()> {
    let hook_path = get_hook_absolute_path()?;

    if !hook_path.exists() {
        let e = Error::msg("Hook is not installed in this repo");
        return Err(e);
    }

    let existing_hook =
        read_to_string(&hook_path).context(format!("Failed to read hook file {:?}", &hook_path))?;

    if existing_hook != CONTENTS {
        let continue_deletion = inquire::Confirm::new(
            "Hook installed is different from this program, do you want to proceed?",
        )
        .with_default(false)
        .prompt()?;

        if !continue_deletion {
            println!("Cancelled");
            return Ok(());
        }
    }

    remove_file(&hook_path).context(format!("Failed to delete hook {:?}", hook_path))?;
    println!("Hook uninstalled for this repo!");

    Ok(())
}
