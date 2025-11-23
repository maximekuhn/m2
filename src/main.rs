use std::str::FromStr;

use serde::Deserialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} path/to/config/file", args[0]);
        std::process::exit(1);
    }

    let config_file_data = std::fs::read_to_string(&args[1])?;
    let config: Config = config_file_data.parse()?;

    let db_pool = m2::db::setup_database(&config.db_file).await?;
    let state = m2::state::AppState::new(db_pool);

    let router = m2::router::app_router(state, config.swagger_enabled);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:55432").await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename(deserialize = "database"))]
    pub db_file: String,

    #[serde(rename(deserialize = "swagger-enabled"))]
    pub swagger_enabled: bool,
}

impl FromStr for Config {
    type Err = serde_yaml_ng::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_yaml_ng::from_str(s)
    }
}
