#[derive(Debug, Clone, clap::Args)]
pub struct DatabaseOptions {
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "MAX_CONNECTIONS", default_value_t = 10)]
    pub max_connections: u32,

    #[arg(long, env = "MIN_CONNECTIONS", default_value_t = 1)]
    pub min_connections: u32,

    #[arg(long, env = "LOG_QUERIES", default_value_t = false)]
    pub log_queries: bool,
}
