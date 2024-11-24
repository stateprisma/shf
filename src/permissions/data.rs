use tokio::sync::{OnceCell, RwLock};

use super::{entries::PermissionsConfig, load::load_permissions};

pub static PERMISSIONS: RwLock<OnceCell<PermissionsConfig>> =
    RwLock::const_new(OnceCell::const_new());

pub async fn initialize_permissions(path: &str) {
    let config = load_permissions(path);
    if let Some(inner) = config {
        PERMISSIONS
            .blocking_write()
            .set(inner)
            .expect("Permissions already initialized");
        return;
    }

    PERMISSIONS
        .write()
        .await
        .set(PermissionsConfig::default())
        .expect("Couldn't set default permission config");
}
