use crate::contracts::TranscriptionAdvancedOptions;
use crate::errors::FrontendError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub model_path: Option<PathBuf>,
    pub preferred_language: Option<String>,
    pub start_in_background: bool,
    pub launch_on_login: bool,
    pub default_preset: Option<String>,
    pub presets: HashMap<String, TranscriptionPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptionPreset {
    pub name: String,
    pub advanced: TranscriptionAdvancedOptions,
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

    pub fn set_start_in_background(&self, enabled: bool) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config.start_in_background = enabled;
        self.save(&config)?;
        Ok(config)
    }

    pub fn set_launch_on_login(&self, enabled: bool) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config.launch_on_login = enabled;
        self.save(&config)?;
        Ok(config)
    }

    pub fn save_preset(
        &self,
        name: String,
        advanced: TranscriptionAdvancedOptions,
    ) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config
            .presets
            .insert(name.clone(), TranscriptionPreset { name, advanced });
        self.save(&config)?;
        Ok(config)
    }

    pub fn delete_preset(&self, name: &str) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config.presets.remove(name);
        if config.default_preset.as_deref() == Some(name) {
            config.default_preset = None;
        }
        self.save(&config)?;
        Ok(config)
    }

    pub fn set_default_preset(&self, name: Option<String>) -> Result<AppConfig, FrontendError> {
        let mut config = self.load()?;
        config.default_preset = name;
        self.save(&config)?;
        Ok(config)
    }

    pub fn get_preset(&self, name: &str) -> Option<TranscriptionPreset> {
        self.load().ok()?.presets.get(name).cloned()
    }

    pub fn list_presets(&self) -> Result<Vec<TranscriptionPreset>, FrontendError> {
        let config = self.load()?;
        let mut presets: Vec<_> = config.presets.values().cloned().collect();
        presets.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(presets)
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
            start_in_background: true,
            launch_on_login: false,
            ..Default::default()
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

    #[test]
    fn save_and_list_presets() {
        use crate::contracts::TranscriptionAdvancedOptions;

        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let advanced = TranscriptionAdvancedOptions {
            task: Some("translate".to_owned()),
            language: Some("fr".to_owned()),
            threads: Some(8),
            ..Default::default()
        };

        store
            .save_preset("French Translate".to_owned(), advanced.clone())
            .expect("save_preset should succeed");

        let presets = store.list_presets().expect("list_presets should succeed");
        assert_eq!(presets.len(), 1);
        assert_eq!(presets[0].name, "French Translate");
        assert_eq!(presets[0].advanced.task, Some("translate".to_owned()));
    }

    #[test]
    fn delete_preset_removes_from_list() {
        use crate::contracts::TranscriptionAdvancedOptions;

        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let advanced = TranscriptionAdvancedOptions::default();
        store
            .save_preset("Test Preset".to_owned(), advanced)
            .expect("save_preset should succeed");

        assert_eq!(store.list_presets().expect("list should work").len(), 1);

        store
            .delete_preset("Test Preset")
            .expect("delete_preset should succeed");

        assert_eq!(store.list_presets().expect("list should work").len(), 0);
    }

    #[test]
    fn set_default_preset_persists() {
        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        store
            .set_default_preset(Some("My Default".to_owned()))
            .expect("set_default_preset should succeed");

        let config = store.load().expect("load should succeed");
        assert_eq!(config.default_preset, Some("My Default".to_owned()));
    }

    #[test]
    fn deleting_default_preset_clears_reference() {
        use crate::contracts::TranscriptionAdvancedOptions;

        let path = unique_config_path();
        let store = ConfigStore::new(&path);

        let advanced = TranscriptionAdvancedOptions::default();
        store
            .save_preset("Default Preset".to_owned(), advanced)
            .expect("save_preset should succeed");
        store
            .set_default_preset(Some("Default Preset".to_owned()))
            .expect("set_default_preset should succeed");

        store
            .delete_preset("Default Preset")
            .expect("delete_preset should succeed");

        let config = store.load().expect("load should succeed");
        assert_eq!(config.default_preset, None);
    }
}
