use std::sync::Arc;

use clap::Parser;
use crabywave::{
    application::http::{HttpServer, HttpServerConfig},
    infrastructure::env::{AppEnv, Env},
};

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt::init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let env = Arc::new(Env::parse());

    init_logger(Arc::clone(&env));

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await?;

    Ok(())
}
