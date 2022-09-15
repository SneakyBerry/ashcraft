use redis::{AsyncCommands, RedisResult};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new() -> RedisResult<RedisClient> {
        Ok(RedisClient {
            client: redis::Client::open("redis://127.0.0.1")?,
        })
    }

    pub async fn set<T>(&self, key: &str, data: &T) -> anyhow::Result<()>
    where
        T: Serialize + Storable,
    {
        let mut conn = self.client.get_async_connection().await?;
        Ok(conn
            .set(
                format!("{}__{}", T::key_prefix(), key),
                serde_json::to_string(data)?,
            )
            .await?)
    }

    pub async fn get<T>(&self, key: &str) -> anyhow::Result<T>
    where
        T: DeserializeOwned + Storable,
    {
        let mut conn = self.client.get_async_connection().await?;
        let key = format!("{}__{}", T::key_prefix(), key);
        let data: Vec<u8> = conn.get(key.clone()).await?;
        Ok(serde_json::from_slice(&data)?)
    }
}

pub trait Storable {
    fn key_prefix() -> &'static str;
}
