use crate::error::Result;
use crate::types::{ObjectInfo, ObjectOptions};
use bytes::Bytes;

pub trait ConfigAPI {
    async fn read_config(&self, key: &str) -> Result<Bytes>;
    async fn read_config_with_metadata(&self, key: &str) -> Result<(Bytes, ObjectInfo)>;
    async fn save_config(&self, key: &str, value: Bytes) -> Result<()>;
    async fn save_config_with_opts(&self, key: &str, value: Bytes, opts: ObjectOptions) -> Result<()>;
    async fn delete_config(&self, key: &str) -> Result<()>;
}
