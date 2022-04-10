use redis::{AsyncCommands, Commands, RedisResult};
use serde::{Deserialize, Serialize};

pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new() -> RedisResult<RedisClient> {
        Ok(RedisClient {
            client: redis::Client::open("redis://127.0.0.1")?,
        })
    }

    pub async fn write<T>(&self, key: &str, data: &T) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.client.get_async_connection().await?;
        Ok(conn.set(key, serde_json::to_string(data)?).await?)
    }

    pub async fn get<T>(&self, key: &str) -> anyhow::Result<T>
    where
        T: Deserialize<'static>,
    {
        let mut conn = self.client.get_async_connection().await?;
        let data: String = conn.get(key).await?;
        Ok(serde_json::from_str(&data)?)
    }
}
