use crate::config::{database::DatabaseOptions, redis::RedisOptions};

#[derive(Debug, Clone, clap::Args)]
pub struct ServerOptions {
    #[arg(short, long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[arg(short, long, env = "PORT", default_value_t = 3000)]
    pub port: u16,

    #[command(flatten)]
    pub database: DatabaseOptions,

    #[command(flatten)]
    pub redis: RedisOptions,
}
