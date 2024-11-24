use axum::Router;

const SERVER_PORT: &str = "3000";
const SERVER_HOST: &str = "0.0.0.0";

const FRONTEND: &str = "public";

mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes::svelte(FRONTEND));

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
        .expect("Expect shutdown signal handler");
    println!("[offline] fsh");
}
