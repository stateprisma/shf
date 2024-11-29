use glob::{Pattern, PatternError};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct PermissionsConfigDe {
    default: UserPermissionsDe,
    tokens: HashMap<String, UserPermissionsDe>,
}

#[derive(Debug)]
pub struct PermissionsConfig {
    default: UserPermissions,
    tokens: HashMap<String, UserPermissions>,
}

impl PermissionsConfig {
    pub fn from_de(raw: PermissionsConfigDe) -> Result<Self, PatternError> {
        let default = UserPermissions::from_de(raw.default)?;
        let mut tokens: HashMap<String, UserPermissions> = HashMap::new();
        for (k, v) in raw.tokens {
            tokens.insert(k, UserPermissions::from_de(v)?);
        }
        Ok(Self { default, tokens })
    }
}

impl Default for PermissionsConfig {
    fn default() -> Self {
        Self {
            default: UserPermissions {
                readonly: None,
                writeable: None,
                // Unwrap here is sound because of static pattern
                hidden: Some(Pattern::new("**").unwrap()),
                except: HashMap::new(),
            },
            tokens: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserPermissionsDe {
    readonly: Option<String>,
    writeable: Option<String>,
    hidden: Option<String>,
    except: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct UserPermissions {
    readonly: Option<Pattern>,
    writeable: Option<Pattern>,
    hidden: Option<Pattern>,
    except: HashMap<Pattern, String>,
}

impl UserPermissions {
    pub fn from_de(raw: UserPermissionsDe) -> Result<Self, PatternError> {
        let readonly = if let Some(pattern_raw) = raw.readonly {
            Some(Pattern::new(&pattern_raw)?)
        } else {
            None
        };
        let writeable = if let Some(pattern_raw) = raw.writeable {
            Some(Pattern::new(&pattern_raw)?)
        } else {
            None
        };
        let hidden = if let Some(raw_pattern) = raw.hidden {
            Some(Pattern::new(&raw_pattern)?)
        } else {
            None
        };
        let mut except: HashMap<Pattern, String> = HashMap::new();
        if let Some(raw_except) = raw.except {
            for (k, v) in &raw_except {
                except.insert(Pattern::new(k)?, v.to_owned());
            }
        };

        Ok(Self {
            readonly,
            writeable,
            hidden,
            except,
        })
    }
}
