use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum HealScanMode {
    #[default]
    Unknown,
    Normal,
    Deep,
}

impl HealScanMode {
    pub fn from_usize(mode: usize) -> Self {
        match mode {
            1 => Self::Normal,
            2 => Self::Deep,
            _ => Self::Unknown,
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::Unknown => 0,
            Self::Normal => 1,
            Self::Deep => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct HealOpts {
    pub recursive: bool,
    #[serde(rename = "dryRun")]
    pub dry_run: bool,
    pub remove: bool,
    pub recreate: bool,
    #[serde(rename = "scanMode")]
    pub scan_mode: HealScanMode,
    #[serde(rename = "updateParity")]
    pub update_parity: bool,
    #[serde(rename = "nolock")]
    pub no_lock: bool,
    pub pool: Option<usize>,
    pub set: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealStartSuccess {
    #[serde(rename = "clientToken")]
    pub client_token: String,
    #[serde(rename = "clientAddress")]
    pub client_address: String,
    #[serde(rename = "startTime")]
    pub start_time: OffsetDateTime,
}

impl Default for HealStartSuccess {
    fn default() -> Self {
        Self {
            client_token: Default::default(),
            client_address: Default::default(),
            start_time: OffsetDateTime::now_utc(),
        }
    }
}

type HealStatusSummary = String;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HealSequenceStatus {
    pub summary: HealStatusSummary,
    pub failure_detail: String,
    pub start_time: u64,
    pub heal_setting: HealOpts,
    pub items: Vec<HealResultItem>,
}

#[derive(Debug, Default)]
pub struct HealSource {
    pub bucket: String,
    pub object: String,
    pub version_id: String,
    pub no_wait: bool,
    pub opts: Option<HealOpts>,
}

pub type HealItemType = String;
type ItemsMap = HashMap<HealItemType, usize>;

#[derive(Debug)]
pub struct HealSequence {
    pub bucket: String,
    pub object: String,
    pub report_progress: bool,
    pub start_time: OffsetDateTime,
    pub end_time: Arc<RwLock<OffsetDateTime>>,
    pub client_token: String,
    pub client_address: String,
    pub force_started: bool,
    pub setting: HealOpts,
    pub current_status: Arc<RwLock<HealSequenceStatus>>,
    pub last_sent_result_index: RwLock<usize>,
    pub scanned_items_map: RwLock<ItemsMap>,
    pub healed_items_map: RwLock<ItemsMap>,
    pub heal_failed_items_map: RwLock<ItemsMap>,
    pub last_heal_activity: RwLock<OffsetDateTime>,

    traverse_and_heal_done_tx: Arc<RwLock<tokio::sync::mpsc::Sender<Option<Error>>>>,
    traverse_and_heal_done_rx: Arc<RwLock<tokio::sync::mpsc::Receiver<Option<Error>>>>,

    tx: tokio::sync::watch::Sender<bool>,
    rx: tokio::sync::watch::Receiver<bool>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HealDriveInfo {
    pub uuid: String,
    pub endpoint: String,
    pub state: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Infos {
    #[serde(rename = "drives")]
    pub drives: Vec<HealDriveInfo>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HealResultItem {
    #[serde(rename = "resultId")]
    pub result_index: usize,
    #[serde(rename = "type")]
    pub heal_item_type: HealItemType,
    #[serde(rename = "bucket")]
    pub bucket: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "detail")]
    pub detail: String,
    #[serde(rename = "parityBlocks")]
    pub parity_blocks: usize,
    #[serde(rename = "dataBlocks")]
    pub data_blocks: usize,
    #[serde(rename = "diskCount")]
    pub disk_count: usize,
    #[serde(rename = "setCount")]
    pub set_count: usize,
    #[serde(rename = "before")]
    pub before: Infos,
    #[serde(rename = "after")]
    pub after: Infos,
    #[serde(rename = "objectSize")]
    pub object_size: usize,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct HealingTracker {
    pub id: String,
    pub pool_index: Option<usize>,
    pub set_index: Option<usize>,
    pub disk_index: Option<usize>,
    pub path: String,
    pub endpoint: String,
    pub started: Option<OffsetDateTime>,
    pub last_update: Option<OffsetDateTime>,
    pub objects_total_count: u64,
    pub objects_total_size: u64,
    pub items_healed: u64,
    pub items_failed: u64,
    pub item_skipped: u64,
    pub bytes_done: u64,
    pub bytes_failed: u64,
    pub bytes_skipped: u64,
    pub bucket: String,
    pub object: String,
    pub resume_items_healed: u64,
    pub resume_items_failed: u64,
    pub resume_items_skipped: u64,
    pub resume_bytes_done: u64,
    pub resume_bytes_failed: u64,
    pub resume_bytes_skipped: u64,
    pub queue_buckets: Vec<String>,
    pub healed_buckets: Vec<String>,
    pub heal_id: String,
    pub retry_attempts: u64,
    pub finished: bool,
}

pub trait HealTrackerApi {
    fn marshal_msg(&self) -> Result<Vec<u8>>;
    fn unmarshal_msg(data: &[u8]) -> Result<HealingTracker>;
}
