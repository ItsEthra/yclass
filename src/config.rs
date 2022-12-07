use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct YClassConfig {
    pub last_attached_process_name: Option<String>,
    #[cfg(debug_assertions)]
    pub last_address: Option<usize>,

    pub plugin_path: Option<PathBuf>,
    pub recent_projects: HashSet<PathBuf>,
    pub dpi: Option<f32>,
}

impl YClassConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .map(|dir| dir.join("yclass/config.toml"))
            .unwrap_or_else(|| "./config.toml".into())
    }

    pub fn load_or_default() -> Self {
        let path = Self::config_path();

        if fs::metadata(&path).is_ok() {
            toml::from_str(&fs::read_to_string(&path).unwrap()).unwrap_or_default()
        } else {
            let value = Self::default();
            if let Some(p) = path.parent() {
                fs::create_dir_all(dbg!(p)).unwrap();
            }

            fs::write(&path, toml::to_string(&value).unwrap().as_bytes()).unwrap();
            value
        }
    }

    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).unwrap();
        }

        fs::write(&path, toml::to_string(self).unwrap().as_bytes()).unwrap();
    }
}
