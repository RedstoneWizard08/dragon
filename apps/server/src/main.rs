#[macro_use]
extern crate log;

use std::{env, net::SocketAddr};

use anyhow::Result;
use api_server::middleware::logger::logging_middleware;
use axum::{middleware::from_fn, Router, Server};
use dotenvy::dotenv;
use glue::{register_glue, GlueOpts};
use include_dir::Dir;

#[cfg(not(debug_assertions))]
pub const CLIENT_DIR: Option<Dir<'static>> = Some(include_dir::include_dir!(
    "$CARGO_MANIFEST_DIR/../client/out"
));

#[cfg(debug_assertions)]
pub const CLIENT_DIR: Option<Dir<'static>> = None;

#[cfg(debug_assertions)]
pub fn start_client() {
    use tokio::process::Command;

    tokio::spawn(async {
        let dir = format!("{}/../client", env!("CARGO_MANIFEST_DIR"));

        Command::new("pnpm")
            .arg("dev")
            .current_dir(dir)
            .spawn()
            .unwrap()
            .wait()
            .await
            .unwrap()
    });
}

#[tokio::main]
pub async fn main() -> Result<()> {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!(
        "==== STARTUP: {} version {} ====",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let ip: [u8; 4] = [127, 0, 0, 1];
    let port = 4000;

    let address = SocketAddr::from((ip, port));
    let server = Server::bind(&address);
    let router = Router::new();

    let router = register_glue(
        router,
        GlueOpts::new()
            .base("http://localhost:4001")
            .dir(CLIENT_DIR),
    );

    let router = router.layer(from_fn(logging_middleware));

    let service = router.into_make_service_with_connect_info::<SocketAddr>();

    #[cfg(debug_assertions)]
    {
        info!("Starting client...");

        start_client();

        info!("Started client!");
    }

    let app = server.serve(service);

    info!("Listening on {}", address);

    app.await.map_err(|v| v.into())
}
