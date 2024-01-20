//
// Partially borrowed from https://gitlab.com/ogarcia/lazycc/-/blob/master/src/config.rs
// SPDX: GPL-3.0-only
//
use {
    super::repo,
    anyhow::{anyhow, Context, Result},
    once_cell::sync::Lazy,
    resolve_path::PathResolveExt,
    serde::Deserialize,
    std::{collections::HashMap, fs::read_to_string, path::PathBuf},
};

const CONFIG_FILENAME: &str = ".commit-emoji.toml"; // for git hooks this is in the GIT_DIR/repo root
const GLOBAL_CONFIG: &str = "~/.config/commit-emoji/config.toml";

#[derive(Clone)]
pub struct Config {
    pub breaking: bool,
    pub commit_types: HashMap<String, String>, // Ignore descriptions
    pub replacements: HashMap<String, String>,
}

fn map_file_config_to_config(from_file: &FileConfig, from_default: &Config) -> Config {
    let mut out_config = from_default.clone();
    if let Some(breaking) = from_file.breaking {
        out_config.breaking = breaking;
    }
    if let Some(ref commit_types) = from_file.commit_types {
        for t in commit_types {
            out_config
                .commit_types
                .entry(t.name.clone())
                .and_modify(|e| *e = t.emoji.clone())
                .or_insert(t.emoji.clone());
        }
    }
    if let Some(ref replacements) = from_file.replacements {
        for r in replacements {
            out_config
                .replacements
                .entry(r.from.clone())
                .and_modify(|e| *e = r.to.clone())
                .or_insert(r.to.clone());
        }
    }
    out_config
}

#[derive(Deserialize)]
struct FileConfig {
    pub breaking: Option<bool>,
    pub commit_types: Option<Vec<EmojiMap>>,
    pub replacements: Option<Vec<ReplacementMap>>,
}

#[derive(Deserialize)]
struct EmojiMap {
    pub name: String,
    pub emoji: String,
    #[allow(dead_code)]
    pub description: Option<String>,
}

/// If entire commit message matches text `from` replace it entirely with the text `to`
#[derive(Deserialize)]
struct ReplacementMap {
    pub from: String,
    pub to: String,
}

fn config_from_file(path: &PathBuf) -> Result<FileConfig> {
    if path.exists() {
        let config_str =
            read_to_string(&path).context(format!("Failed to read config file {:?}", &path))?;
        let config: FileConfig = toml::from_str(&config_str)
            .context(format!("Failed to parse config file {:?}", &path))?;
        return Ok(config);
    }
    return Err(anyhow!("Failed to read config from {}", path.display()));
}

pub fn config() -> Result<Config> {
    let mut path = repo::get_absolute_path()?;
    path.pop();
    path.push(CONFIG_FILENAME);

    let file_config = config_from_file(&path)
        .or_else(|_| config_from_file(&PathBuf::from(GLOBAL_CONFIG.resolve())))
        .or_else(|_| {
            Ok::<FileConfig, anyhow::Error>(FileConfig {
                breaking: None,
                commit_types: None,
                replacements: None,
            })
        })
        .unwrap();

    Ok(map_file_config_to_config(&file_config, &*DEFAULT_CONFIG))
}

// A default config of emoji based on https://gist.github.com/berkus/5ce2cdf5dd74909bcd4faf6cb7d0ae18
// Changes to it can be made via a config file (~/.config/commit-emoji/config.toml or .commit-emoji.toml in repo dir)

static DEFAULT_CONFIG: Lazy<Config> = Lazy::new(|| Config {
    breaking: true,
    commit_types: HashMap::from([
        ("feat".into(), "✨".into()),
        ("fix".into(), "🐛".into()),
        ("docs".into(), "📚".into()),
        ("style".into(), "💎".into()),
        ("refactor".into(), "📦".into()),
        ("perf".into(), "🚀".into()),
        ("test".into(), "🚨".into()),
        ("build".into(), "🛠".into()),
        ("ci".into(), "⚙️".into()),
        ("chore".into(), "♻️".into()),
        ("revert".into(), "🗑".into()),
        ("deprecated".into(), "⛔️".into()),
        ("removed".into(), "❌".into()),
        ("security".into(), "🔐".into()),
        ("merge".into(), "⛓️".into()),
        ("breaking".into(), "💔".into()),
    ]),
    replacements: HashMap::from([
        ("initial".into(), "feat: 🎉 Initial commit".into()),
        ("deps".into(), "fix: ⏫ Update dependencies".into()),
        ("peerdeps".into(), "fix: ⬆️ Update peer dependencies".into()),
        (
            "devdeps".into(),
            "chore: 🔼 Update development dependencies".into(),
        ),
        (
            "metadata".into(),
            "fix: 📦 Update metadata (Cargo.toml)".into(),
        ),
    ]),
});
