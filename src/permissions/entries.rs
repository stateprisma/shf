use glob::Pattern;
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
                readonly: None,
                writeable: None,
                // Unwrap here is sound because of static pattern
                hidden: Some(Pattern::new("**").unwrap()),
                except: None,
            },
            tokens: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserPermissions {
    readonly: Option<Pattern>,
    writeable: Option<Pattern>,
    hidden: Option<Pattern>,
    except: Option<HashMap<String, String>>,
}
