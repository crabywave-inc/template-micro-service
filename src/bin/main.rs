use std::sync::Arc;

use clap::Parser;
use infrastructure::env::{AppEnv, Env};

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

    println!("env: {:?}", &env);

    Ok(())
}
