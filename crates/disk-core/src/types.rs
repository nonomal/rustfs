// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::error::Error;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

pub const RUSTFS_META_BUCKET: &str = ".rustfs.sys";
pub const RUSTFS_META_MULTIPART_BUCKET: &str = ".rustfs.sys/multipart";
pub const RUSTFS_META_TMP_BUCKET: &str = ".rustfs.sys/tmp";
pub const RUSTFS_META_TMP_DELETED_BUCKET: &str = ".rustfs.sys/tmp/.trash";
pub const BUCKET_META_PREFIX: &str = "buckets";
pub const FORMAT_CONFIG_FILE: &str = "format.json";
pub const STORAGE_FORMAT_FILE: &str = "xl.meta";
pub const STORAGE_FORMAT_FILE_BACKUP: &str = "xl.meta.bkp";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CheckPartsResp {
    pub results: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateMetadataOpts {
    pub no_persistence: bool,
}

pub struct DiskLocation {
    pub pool_idx: Option<usize>,
    pub set_idx: Option<usize>,
    pub disk_idx: Option<usize>,
}

impl DiskLocation {
    pub fn valid(&self) -> bool {
        self.pool_idx.is_some() && self.set_idx.is_some() && self.disk_idx.is_some()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DiskInfoOptions {
    pub disk_id: String,
    pub metrics: bool,
    pub noop: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiskInfo {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub used_inodes: u64,
    pub free_inodes: u64,
    pub major: u64,
    pub minor: u64,
    pub nr_requests: u64,
    pub fs_type: String,
    pub root_disk: bool,
    pub healing: bool,
    pub scanning: bool,
    pub endpoint: String,
    pub mount_path: String,
    pub id: String,
    pub rotational: bool,
    pub error: String,
}

#[derive(Clone, Debug, Default)]
pub struct Info {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub files: u64,
    pub ffree: u64,
    pub fstype: String,
    pub major: u64,
    pub minor: u64,
    pub name: String,
    pub rotational: bool,
    pub nrrequests: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WalkDirOptions {
    // Bucket to scanner
    pub bucket: String,
    // Directory inside the bucket.
    pub base_dir: String,
    // Do a full recursive scan.
    pub recursive: bool,

    // ReportNotFound will return errFileNotFound if all disks reports the BaseDir cannot be found.
    pub report_notfound: bool,

    // FilterPrefix will only return results with given prefix within folder.
    // Should never contain a slash.
    pub filter_prefix: Option<String>,

    // ForwardTo will forward to the given object path.
    pub forward_to: Option<String>,

    // Limit the number of returned objects if > 0.
    pub limit: i32,

    // DiskID contains the disk ID of the disk.
    // Leave empty to not check disk ID.
    pub disk_id: String,
}

#[derive(Clone, Debug, Default)]
pub struct DiskOption {
    pub cleanup: bool,
    pub health_check: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RenameDataResp {
    pub old_data_dir: Option<Uuid>,
    pub sign: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteOptions {
    pub recursive: bool,
    pub immediate: bool,
    pub undo_write: bool,
    pub old_data_dir: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleReq {
    pub bucket: String,
    pub prefix: String,
    pub files: Vec<String>,
    pub max_size: usize,
    pub metadata_only: bool,
    pub abort404: bool,
    pub max_results: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadMultipleResp {
    pub bucket: String,
    pub prefix: String,
    pub file: String,
    pub exists: bool,
    pub error: String,
    pub data: Vec<u8>,
    pub mod_time: Option<OffsetDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeInfo {
    pub name: String,
    pub created: Option<OffsetDateTime>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ReadOptions {
    pub incl_free_versions: bool,
    pub read_data: bool,
    pub healing: bool,
}

pub const CHECK_PART_UNKNOWN: usize = 0;
// Changing the order can cause a data loss
// when running two nodes with incompatible versions
pub const CHECK_PART_SUCCESS: usize = 1;
pub const CHECK_PART_DISK_NOT_FOUND: usize = 2;
pub const CHECK_PART_VOLUME_NOT_FOUND: usize = 3;
pub const CHECK_PART_FILE_NOT_FOUND: usize = 4;
pub const CHECK_PART_FILE_CORRUPT: usize = 5;

pub fn conv_part_err_to_int(err: &Option<Error>) -> usize {
    match err {
        Some(Error::FileNotFound) | Some(Error::FileVersionNotFound) => CHECK_PART_FILE_NOT_FOUND,
        Some(Error::FileCorrupt) => CHECK_PART_FILE_CORRUPT,
        Some(Error::VolumeNotFound) => CHECK_PART_VOLUME_NOT_FOUND,
        Some(Error::DiskNotFound) => CHECK_PART_DISK_NOT_FOUND,
        None => CHECK_PART_SUCCESS,
        _ => CHECK_PART_UNKNOWN,
    }
}

pub fn has_part_err(part_errs: &[usize]) -> bool {
    part_errs.iter().any(|err| *err != CHECK_PART_SUCCESS)
}
