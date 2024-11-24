use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PermissionsConfig {
    default: UserPermissions,
    tokens: HashMap<String, UserPermissions>,
}

impl Default for PermissionsConfig {
    fn default() -> Self {
        Self {
            default: UserPermissions {
                perm: "r".to_owned(),
                except: None,
            },
            tokens: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserPermissions {
    perm: String,
    except: Option<HashMap<String, String>>,
}
