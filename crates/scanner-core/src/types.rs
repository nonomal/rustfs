use std::collections::HashMap;
use std::time::SystemTime;

use rustfs_madmin::types::BucketLifecycleConfiguration;
use rustfs_madmin::types::ReplicationConfiguration;
use tokio::sync::mpsc::Sender;

use crate::data_usage_cache::DataUsageEntry;

#[derive(Clone)]
pub struct DataUsageEntryInfo {
    pub name: String,
    pub parent: String,
    pub entry: DataUsageEntry,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataUsageCacheInfo {
    pub name: String,
    pub next_cycle: u32,
    pub last_update: Option<SystemTime>,
    pub skip_healing: bool,
    #[serde(skip)]
    pub lifecycle: Option<BucketLifecycleConfiguration>,
    #[serde(skip)]
    pub updates: Option<Sender<DataUsageEntry>>,
    #[serde(skip)]
    pub replication: Option<ReplicationConfiguration>,
}

// impl Default for DataUsageCacheInfo {
//     fn default() -> Self {
//         Self {
//             name: Default::default(),
//             next_cycle: Default::default(),
//             last_update: SystemTime::now(),
//             skip_healing: Default::default(),
//             updates: Default::default(),
//             replication: Default::default(),
//         }
//     }
// }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataUsageCache {
    pub info: DataUsageCacheInfo,
    pub cache: HashMap<String, DataUsageEntry>,
}
