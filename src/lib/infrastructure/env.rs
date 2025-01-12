use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum AppEnv {
    Production,
    #[default]
    Development,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env)]
    pub port: String,

    #[clap(env)]
    pub gogole_project_id: String,

    #[clap(env, default_value = "development", value_enum)]
    pub env: AppEnv,
}
