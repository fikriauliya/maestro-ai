use serde::Deserialize;
use std::fs;
use std::path::Path;

const CONFIG_FILE: &str = ".config/wt.toml";
const INSTALL_MARKER: &str = ".maestro-installed";

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Deserialize, Default)]
pub struct Hooks {
    /// One-time hook: runs only once after fresh worktree creation (e.g., `bun install`)
    pub install: Option<String>,
    /// Start hook: runs every time worktree is opened (e.g., `bun run serve`)
    pub start: Option<String>,
}

impl Config {
    /// Load config from worktree's .config/wt.toml
    pub fn load(worktree_path: &Path) -> Option<Self> {
        let config_path = worktree_path.join(CONFIG_FILE);
        let content = fs::read_to_string(&config_path).ok()?;
        toml::from_str(&content).ok()
    }

    /// Check if install hook has already run for this worktree
    pub fn install_completed(worktree_path: &Path) -> bool {
        worktree_path.join(INSTALL_MARKER).exists()
    }

    /// Mark install hook as completed
    pub fn mark_install_completed(worktree_path: &Path) -> std::io::Result<()> {
        fs::write(worktree_path.join(INSTALL_MARKER), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml = r#"
[hooks]
install = "bun install"
start = "bun run serve"
"#;
        let config: Config = toml::from_str(toml).unwrap();
        assert_eq!(config.hooks.install, Some("bun install".to_string()));
        assert_eq!(config.hooks.start, Some("bun run serve".to_string()));
    }

    #[test]
    fn test_empty_config() {
        let config: Config = toml::from_str("").unwrap();
        assert!(config.hooks.install.is_none());
        assert!(config.hooks.start.is_none());
    }
}
