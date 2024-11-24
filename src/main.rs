use axum::Router;

const SERVER_PORT: &str = "3000";
const SERVER_HOST: &str = "0.0.0.0";

const FRONTEND: &str = "public";
const PERMISSIONS_CONFIG: &str = ".shf.toml";

mod routes;
mod websocket;

mod permissions;

#[tokio::main]
async fn main() {
    /* Load permission config */
    permissions::data::initialize_permissions(PERMISSIONS_CONFIG).await;

    let app = Router::new()
        .merge(routes::svelte(FRONTEND))
        .merge(routes::websocket());

    let listener = tokio::net::TcpListener::bind(format!("{SERVER_HOST}:{SERVER_PORT}"))
        .await
        .unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Shutdown signal handler failed");
    println!("[offl] fsh");
}
