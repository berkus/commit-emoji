//
// Partially borrowed from https://gitlab.com/ogarcia/lazycc/-/blob/master/src/commit.rs
// SPDX: GPL-3.0-only
//
use {
    super::{config::config, repo},
    anyhow::{Context, Result},
    git_conventional::Commit,
    std::{
        fmt,
        fs::{read_to_string, write},
    },
    unic_emoji_char::is_emoji,
};

pub fn check_rebase() -> Result<bool> {
    let path = repo::get_absolute_path()?;

    if path.join("rebase-merge").exists() || path.join("rebase-apply").exists() {
        return Ok(true);
    }

    Ok(false)
}

// Since we cannot construct commit manually, steal and reuse the formatting code.
struct NewCommit<'a>(Commit<'a>, String);

impl fmt::Display for NewCommit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0.type_().as_str())?;

        if let Some(scope) = &self.0.scope() {
            f.write_fmt(format_args!("({})", scope))?;
        }

        f.write_fmt(format_args!(": {}", self.1))?;

        if let Some(body) = &self.0.body() {
            f.write_fmt(format_args!("\n\n{}", body))?;
        }

        for t in self.0.footers() {
            write!(f, "\n\n{}{}{}", t.token(), t.separator(), t.value())?;
        }

        Ok(())
    }
}

pub fn generate_commit_message(commit_msg_file: &str) -> Result<()> {
    let existing_message = read_to_string(&commit_msg_file).context(format!(
        "Failed to read commit message file {:?}",
        &commit_msg_file
    ))?;

    let config = config()?;

    // if entire message is matching "replacements" - replace and be done with it!
    let trimmed_message = existing_message.trim();
    if trimmed_message.lines().count() == 1 {
        if let Some(replacement) = config.replacements.get(trimmed_message) {
            write(commit_msg_file, replacement).context(format!(
                "Failed to write to commit message file {:?}",
                &commit_msg_file
            ))?;
            return Ok(());
        }
    }

    let commit = Commit::parse(&existing_message)?;

    let type_ = commit.type_(); // type must be in config map, if not - return Ok(())
    let desc = commit.description(); // desc must not start with an emoji, it if does - return Ok(())
    if desc.starts_with(|c: char| is_emoji(c)) {
        return Ok(());
    }

    let breaking = commit.breaking();

    let description = if config.breaking && breaking {
        format!("{} {}", config.commit_types["breaking"], desc)
    } else if let Some(e) = config.commit_types.get(type_.as_str()) {
        format!("{} {}", e, desc)
    } else {
        desc.into()
    };

    let new_commit = NewCommit(commit, description);
    let commit_message = format!("{}", new_commit);

    write(commit_msg_file, commit_message).context(format!(
        "Failed to write to commit message file {:?}",
        &commit_msg_file
    ))?;
    Ok(())
}
