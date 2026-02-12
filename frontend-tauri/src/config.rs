use crate::errors::FrontendError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub model_path: Option<PathBuf>,
    pub preferred_language: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<AppConfig, FrontendError> {
        if !self.path.exists() {
            return Ok(AppConfig::default());
        }

        let raw = fs::read_to_string(&self.path).map_err(|source| FrontendError::Io {
            context: "failed to read config file",
            source,
        })?;

        serde_json::from_str(&raw).map_err(|source| FrontendError::Serialization {
            context: "failed to parse config file",
            source,
        })
    }

    pub fn save(&self, config: &AppConfig) -> Result<(), FrontendError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|source| FrontendError::Io {
                context: "failed to create config directory",
                source,
            })?;
        }

        let raw = serde_json::to_string_pretty(config).map_err(|source| {
            FrontendError::Serialization {
                context: "failed to serialize config",
                source,
            }
        })?;

        fs::write(&self.path, raw).map_err(|source| FrontendError::Io {
            context: "failed to write config file",
            source,
        })
    }

    pub fn set_model_path(&self, model_path: PathBuf) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config.model_path = Some(model_path);
        self.save(&config)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, ConfigStore};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_config_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!("frontend-tauri-config-{nanos}.json"))
    }

    #[test]
    fn load_returns_default_when_config_missing() {
        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let loaded = store.load().expect("load should succeed for missing file");
        assert_eq!(loaded, AppConfig::default());
    }

    #[test]
    fn save_then_load_roundtrip() {
        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let config = AppConfig {
            model_path: Some(PathBuf::from("models/ggml-base.en.bin")),
            preferred_language: Some("en".to_owned()),
        };

        store.save(&config).expect("save should succeed");
        let loaded = store.load().expect("load should succeed after save");

        assert_eq!(loaded, config);
    }

    #[test]
    fn set_model_path_updates_config() {
        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let updated = store
            .set_model_path(PathBuf::from("models/ggml-small.bin"))
            .expect("set_model_path should succeed");

        assert_eq!(
            updated.model_path,
            Some(PathBuf::from("models/ggml-small.bin"))
        );
    }
}
