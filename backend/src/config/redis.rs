#[derive(Debug, Clone, clap::Args)]
pub struct RedisOptions {
    #[arg(long, env = "REDIS_URL")]
    pub redis_url: String,

    #[arg(long, env = "REDIS_CONNECTIONS", default_value_t = 10)]
    pub connections: usize,
}
