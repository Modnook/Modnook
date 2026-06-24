use fred::{interfaces::KeysInterface, types::config::Config};

use crate::{cache::core::Cache, config::redis::RedisOptions};

#[derive(Debug, thiserror::Error)]
pub enum RedisCacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] fred::error::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct RedisCache {
    pool: fred::clients::Pool,
}

impl RedisCache {
    pub fn new(options: RedisOptions) -> Result<Self, fred::error::Error> {
        let config = Config::from_url(&options.redis_url)?;
        let pool = fred::types::Builder::from_config(config).build_pool(options.connections)?;

        Ok(Self { pool })
    }
}

impl Cache for RedisCache {
    type Error = RedisCacheError;

    async fn get<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Self::Error> {
        let result: Option<String> = self.pool.get(key).await?;

        match result {
            Some(json_string) => {
                let data: T = serde_json::from_str(&json_string)?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    async fn set<T: serde::Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: i64,
    ) -> Result<(), Self::Error> {
        let json_string = serde_json::to_string(value)?;

        self.pool
            .set::<(), _, _>(
                key,
                json_string,
                Some(fred::types::Expiration::EX(ttl_seconds)),
                None,
                false,
            )
            .await?;

        Ok(())
    }
}
