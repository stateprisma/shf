use std::fs;
use std::path::Path;
use toml;

use super::entries::{PermissionsConfig, PermissionsConfigDe};

pub fn load_permissions(path: &str) -> Option<PermissionsConfig> {
    let config_path = Path::new(path);

    let loaded: Option<PermissionsConfigDe> = if config_path.exists() {
        match fs::read_to_string(config_path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => Some(config),
                Err(err) => {
                    eprintln!("Failed to parse {}: {}", path, err);
                    None
                }
            },
            Err(err) => {
                eprintln!("Failed to read {}: {}", path, err);
                None
            }
        }
    } else {
        /* No config file found */
        None
    };

    if let Some(perms_raw) = loaded {
        Some(PermissionsConfig::default())
    } else {
        None
    }
}
