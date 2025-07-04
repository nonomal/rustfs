use std::collections::HashMap;

use crate::bucket_target::BucketTarget;
use crate::config::ConfigAPI;
use crate::error::{Error, Result};
use crate::object_lock::ObjectLockApi;
use crate::versioning::VersioningApi;
use crate::{bucket_quote::BucketQuota, bucket_target::BucketTargets};
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;
use rmp_serde::Serializer as rmpSerializer;
use rustfs_disk_core::BUCKET_META_PREFIX;
use rustfs_policy::policy::BucketPolicy;
use rustfs_utils::s3s::deserialize;
use s3s::dto::{
    BucketLifecycleConfiguration, NotificationConfiguration, ObjectLockConfiguration, ReplicationConfiguration,
    ServerSideEncryptionConfiguration, Tagging, VersioningConfiguration,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub const BUCKET_METADATA_FILE: &str = ".metadata.bin";
pub const BUCKET_METADATA_FORMAT: u16 = 1;
pub const BUCKET_METADATA_VERSION: u16 = 1;

pub const BUCKET_POLICY_CONFIG: &str = "policy.json";
pub const BUCKET_NOTIFICATION_CONFIG: &str = "notification.xml";
pub const BUCKET_LIFECYCLE_CONFIG: &str = "lifecycle.xml";
pub const BUCKET_SSECONFIG: &str = "bucket-encryption.xml";
pub const BUCKET_TAGGING_CONFIG: &str = "tagging.xml";
pub const BUCKET_QUOTA_CONFIG_FILE: &str = "quota.json";
pub const OBJECT_LOCK_CONFIG: &str = "object-lock.xml";
pub const BUCKET_VERSIONING_CONFIG: &str = "versioning.xml";
pub const BUCKET_REPLICATION_CONFIG: &str = "replication.xml";
pub const BUCKET_TARGETS_FILE: &str = "bucket-targets.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "PascalCase", default)]
pub struct BucketMetadata {
    pub name: String,
    pub created: OffsetDateTime,
    pub lock_enabled: bool, // While marked as unused, it may need to be retained
    pub policy_config_json: Vec<u8>,
    pub notification_config_xml: Vec<u8>,
    pub lifecycle_config_xml: Vec<u8>,
    pub object_lock_config_xml: Vec<u8>,
    pub versioning_config_xml: Vec<u8>,
    pub encryption_config_xml: Vec<u8>,
    pub tagging_config_xml: Vec<u8>,
    pub quota_config_json: Vec<u8>,
    pub replication_config_xml: Vec<u8>,
    pub bucket_targets_config_json: Vec<u8>,
    pub bucket_targets_config_meta_json: Vec<u8>,

    pub policy_config_updated_at: OffsetDateTime,
    pub object_lock_config_updated_at: OffsetDateTime,
    pub encryption_config_updated_at: OffsetDateTime,
    pub tagging_config_updated_at: OffsetDateTime,
    pub quota_config_updated_at: OffsetDateTime,
    pub replication_config_updated_at: OffsetDateTime,
    pub versioning_config_updated_at: OffsetDateTime,
    pub lifecycle_config_updated_at: OffsetDateTime,
    pub notification_config_updated_at: OffsetDateTime,
    pub bucket_targets_config_updated_at: OffsetDateTime,
    pub bucket_targets_config_meta_updated_at: OffsetDateTime,

    #[serde(skip)]
    pub new_field_updated_at: OffsetDateTime,

    #[serde(skip)]
    pub policy_config: Option<BucketPolicy>,
    #[serde(skip)]
    pub notification_config: Option<NotificationConfiguration>,
    #[serde(skip)]
    pub lifecycle_config: Option<BucketLifecycleConfiguration>,
    #[serde(skip)]
    pub object_lock_config: Option<ObjectLockConfiguration>,
    #[serde(skip)]
    pub versioning_config: Option<VersioningConfiguration>,
    #[serde(skip)]
    pub sse_config: Option<ServerSideEncryptionConfiguration>,
    #[serde(skip)]
    pub tagging_config: Option<Tagging>,
    #[serde(skip)]
    pub quota_config: Option<BucketQuota>,
    #[serde(skip)]
    pub replication_config: Option<ReplicationConfiguration>,
    #[serde(skip)]
    pub bucket_target_config: Option<BucketTargets>,
    #[serde(skip)]
    pub bucket_target_config_meta: Option<HashMap<String, String>>,
}

impl Default for BucketMetadata {
    fn default() -> Self {
        Self {
            name: Default::default(),
            created: OffsetDateTime::UNIX_EPOCH,
            lock_enabled: Default::default(),
            policy_config_json: Default::default(),
            notification_config_xml: Default::default(),
            lifecycle_config_xml: Default::default(),
            object_lock_config_xml: Default::default(),
            versioning_config_xml: Default::default(),
            encryption_config_xml: Default::default(),
            tagging_config_xml: Default::default(),
            quota_config_json: Default::default(),
            replication_config_xml: Default::default(),
            bucket_targets_config_json: Default::default(),
            bucket_targets_config_meta_json: Default::default(),
            policy_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            object_lock_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            encryption_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            tagging_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            quota_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            replication_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            versioning_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            lifecycle_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            notification_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            bucket_targets_config_updated_at: OffsetDateTime::UNIX_EPOCH,
            bucket_targets_config_meta_updated_at: OffsetDateTime::UNIX_EPOCH,
            new_field_updated_at: OffsetDateTime::UNIX_EPOCH,
            policy_config: Default::default(),
            notification_config: Default::default(),
            lifecycle_config: Default::default(),
            object_lock_config: Default::default(),
            versioning_config: Default::default(),
            sse_config: Default::default(),
            tagging_config: Default::default(),
            quota_config: Default::default(),
            replication_config: Default::default(),
            bucket_target_config: Default::default(),
            bucket_target_config_meta: Default::default(),
        }
    }
}

impl BucketMetadata {
    pub fn new(name: &str) -> Self {
        BucketMetadata {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn save_file_path(&self) -> String {
        format!("{}/{}/{}", BUCKET_META_PREFIX, self.name.as_str(), BUCKET_METADATA_FILE)
    }

    pub fn versioning(&self) -> bool {
        self.lock_enabled
            || (self.object_lock_config.as_ref().is_some_and(|v| v.enabled())
                || self.versioning_config.as_ref().is_some_and(|v| v.enabled()))
    }

    pub fn object_locking(&self) -> bool {
        self.lock_enabled || (self.versioning_config.as_ref().is_some_and(|v| v.enabled()))
    }

    pub fn marshal_msg(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();

        self.serialize(&mut rmpSerializer::new(&mut buf).with_struct_map())?;

        Ok(buf)
    }

    pub fn unmarshal(buf: &[u8]) -> Result<Self> {
        let t: BucketMetadata = rmp_serde::from_slice(buf)?;
        Ok(t)
    }

    pub fn check_header(buf: &[u8]) -> Result<()> {
        if buf.len() <= 4 {
            return Err(Error::other("read_bucket_metadata: data invalid"));
        }

        let format = LittleEndian::read_u16(&buf[0..2]);
        let version = LittleEndian::read_u16(&buf[2..4]);

        match format {
            BUCKET_METADATA_FORMAT => {}
            _ => return Err(Error::other("read_bucket_metadata: format invalid")),
        }

        match version {
            BUCKET_METADATA_VERSION => {}
            _ => return Err(Error::other("read_bucket_metadata: version invalid")),
        }

        Ok(())
    }

    fn default_timestamps(&mut self) {
        if self.policy_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.policy_config_updated_at = self.created
        }
        if self.encryption_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.encryption_config_updated_at = self.created
        }

        if self.tagging_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.tagging_config_updated_at = self.created
        }
        if self.object_lock_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.object_lock_config_updated_at = self.created
        }
        if self.quota_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.quota_config_updated_at = self.created
        }

        if self.replication_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.replication_config_updated_at = self.created
        }

        if self.versioning_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.versioning_config_updated_at = self.created
        }

        if self.lifecycle_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.lifecycle_config_updated_at = self.created
        }
        if self.notification_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.notification_config_updated_at = self.created
        }

        if self.bucket_targets_config_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.bucket_targets_config_updated_at = self.created
        }
        if self.bucket_targets_config_meta_updated_at == OffsetDateTime::UNIX_EPOCH {
            self.bucket_targets_config_meta_updated_at = self.created
        }
    }

    pub fn update_config(&mut self, config_file: &str, data: Vec<u8>) -> Result<OffsetDateTime> {
        let updated = OffsetDateTime::now_utc();

        match config_file {
            BUCKET_POLICY_CONFIG => {
                self.policy_config_json = data;
                self.policy_config_updated_at = updated;
            }
            BUCKET_NOTIFICATION_CONFIG => {
                self.notification_config_xml = data;
                self.notification_config_updated_at = updated;
            }
            BUCKET_LIFECYCLE_CONFIG => {
                self.lifecycle_config_xml = data;
                self.lifecycle_config_updated_at = updated;
            }
            BUCKET_SSECONFIG => {
                self.encryption_config_xml = data;
                self.encryption_config_updated_at = updated;
            }
            BUCKET_TAGGING_CONFIG => {
                self.tagging_config_xml = data;
                self.tagging_config_updated_at = updated;
            }
            BUCKET_QUOTA_CONFIG_FILE => {
                self.quota_config_json = data;
                self.quota_config_updated_at = updated;
            }
            OBJECT_LOCK_CONFIG => {
                self.object_lock_config_xml = data;
                self.object_lock_config_updated_at = updated;
            }
            BUCKET_VERSIONING_CONFIG => {
                self.versioning_config_xml = data;
                self.versioning_config_updated_at = updated;
            }
            BUCKET_REPLICATION_CONFIG => {
                self.replication_config_xml = data;
                self.replication_config_updated_at = updated;
            }
            BUCKET_TARGETS_FILE => {
                self.bucket_targets_config_json = data.clone();
                self.bucket_targets_config_updated_at = updated;
            }
            _ => return Err(Error::other(format!("config file not found : {config_file}"))),
        }

        Ok(updated)
    }

    pub fn set_created(&mut self, created: Option<OffsetDateTime>) {
        self.created = created.unwrap_or_else(OffsetDateTime::now_utc)
    }

    pub async fn save<S: ConfigAPI>(&mut self, store: S) -> Result<()> {
        self.parse_all_configs()?;

        let mut buf: Vec<u8> = vec![0; 4];

        LittleEndian::write_u16(&mut buf[0..2], BUCKET_METADATA_FORMAT);

        LittleEndian::write_u16(&mut buf[2..4], BUCKET_METADATA_VERSION);

        let data = self.marshal_msg()?;

        buf.extend_from_slice(&data);

        store.save_config(self.save_file_path().as_str(), Bytes::from(buf)).await?;

        Ok(())
    }

    fn parse_all_configs(&mut self) -> Result<()> {
        if !self.policy_config_json.is_empty() {
            self.policy_config = Some(serde_json::from_slice(&self.policy_config_json)?);
        }
        if !self.notification_config_xml.is_empty() {
            self.notification_config = Some(deserialize::<NotificationConfiguration>(&self.notification_config_xml)?);
        }
        if !self.lifecycle_config_xml.is_empty() {
            self.lifecycle_config = Some(deserialize::<BucketLifecycleConfiguration>(&self.lifecycle_config_xml)?);
        }

        if !self.object_lock_config_xml.is_empty() {
            self.object_lock_config = Some(deserialize::<ObjectLockConfiguration>(&self.object_lock_config_xml)?);
        }
        if !self.versioning_config_xml.is_empty() {
            self.versioning_config = Some(deserialize::<VersioningConfiguration>(&self.versioning_config_xml)?);
        }
        if !self.encryption_config_xml.is_empty() {
            self.sse_config = Some(deserialize::<ServerSideEncryptionConfiguration>(&self.encryption_config_xml)?);
        }
        if !self.tagging_config_xml.is_empty() {
            self.tagging_config = Some(deserialize::<Tagging>(&self.tagging_config_xml)?);
        }
        if !self.quota_config_json.is_empty() {
            self.quota_config = Some(BucketQuota::unmarshal(&self.quota_config_json)?);
        }
        if !self.replication_config_xml.is_empty() {
            self.replication_config = Some(deserialize::<ReplicationConfiguration>(&self.replication_config_xml)?);
        }
        //let temp = self.bucket_targets_config_json.clone();
        if !self.bucket_targets_config_json.is_empty() {
            let arr: Vec<BucketTarget> = serde_json::from_slice(&self.bucket_targets_config_json)?;
            self.bucket_target_config = Some(BucketTargets { targets: arr });
        } else {
            self.bucket_target_config = Some(BucketTargets::default())
        }

        Ok(())
    }
}
