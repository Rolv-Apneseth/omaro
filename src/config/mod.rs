use std::{fs::read_to_string, path::{Path, PathBuf}, sync::LazyLock};

use color_eyre::{Result, Section, eyre::{Context, eyre}};
use dirs::config_dir;
use serde::Deserialize;

mod components;
pub use components::*;

use crate::modes::Mode;

pub static DEFAULT_CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    config_dir()
        .expect("could not identify config directory")
        .join("omaro")
        .join("config.toml")
});

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    #[serde(default)]
    pub default_mode: Mode,
    #[serde(default = "_default_true")]
    pub opening_comments_marks_posts_read: bool,
    #[serde(default = "_default_true")]
    pub previewing_comments_marks_posts_read: bool,

    pub ui: UiConfig,
}

fn _default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_mode: Default::default(),
            ui: UiConfig::default(),
            opening_comments_marks_posts_read: true,
            previewing_comments_marks_posts_read: true,
        }
    }
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[serde(default)]
pub struct UiConfig {
    pub borders: BordersConfig,
    pub header: HeaderConfig,
    pub comment_count: CommentCountConfig,
    pub submitted_elapsed: SubmittedElapsedConfig,
    pub submitted_user: SubmittedUserConfig,
    pub scrollbar: ScrollbarConfig,
    pub score_count: ScoreCountConfig,
    pub shortcuts: ShortcutsUiConfig,
    pub downloaded: DownloadedConfig,
    pub keybind_hints: KeybindHintsConfig,
    pub mode_info: ModeInfoConfig,
}

pub fn get_config(config: &Path, clean: bool) -> Result<Config> {
    if clean {
        return Ok(Config::default());
    }

    let is_default = config == DEFAULT_CONFIG_PATH.as_path();
    if !config.is_file() {
        if is_default {
            // debug!("Config file not found at {path_config:?}");
            return Ok(Config::default());
        }

        return Err(eyre!(format!(
            "Not a valid file: {}",
            config.to_string_lossy()
        )));
    }

    // debug!("Config file found at {:?}", &path_config);
    let contents = read_to_string(config)
        .context("Could not read config file contents")
        .suggestion("Ensure that the file is valid UTF-8 and has the appropriate permissions")?;

    toml::from_str::<Config>(&contents)
        .context("Failed to parse toml")
        .suggestion("Ensure that the configuration file is valid")
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn clean_is_default() {
        let default_config = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("configs")
            .join("default.toml");
        assert_eq!(
            get_config(&default_config, false).unwrap(),
            get_config(&PathBuf::new(), true).unwrap()
        );
    }

    #[test]
    fn invalid_config_errors() {
        assert!(get_config(&PathBuf::new(), false).is_err());
        assert!(
            get_config(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
                false
            )
            .is_err()
        );
    }

    #[test]
    fn all_template_configs_valid() {
        let configs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("configs");

        assert!(get_config(&configs_dir.join("default.toml"), false).is_ok());
        assert!(get_config(&configs_dir.join("minimal.toml"), false).is_ok());
        assert!(get_config(&configs_dir.join("minimaler.toml"), false).is_ok());
        assert!(get_config(&configs_dir.join("minimalest.toml"), false).is_ok());
    }
}
