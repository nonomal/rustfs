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

use crate::error::{Error, Result};
use crate::types::*;
use bytes::Bytes;
use rustfs_endpoints::Endpoint;
use rustfs_filemeta::{FileInfo, RawFileInfo};
use std::fmt::Debug;
use std::path::PathBuf;
use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;

pub type FileReader = Box<dyn AsyncRead + Send + Sync + Unpin>;
pub type FileWriter = Box<dyn AsyncWrite + Send + Sync + Unpin>;

#[async_trait::async_trait]
pub trait DiskAPI: Debug + Send + Sync + 'static {
    fn to_string(&self) -> String;
    async fn is_online(&self) -> bool;
    fn is_local(&self) -> bool;
    // LastConn
    fn host_name(&self) -> String;
    fn endpoint(&self) -> Endpoint;
    async fn close(&self) -> Result<()>;
    async fn get_disk_id(&self) -> Result<Option<Uuid>>;
    async fn set_disk_id(&self, id: Option<Uuid>) -> Result<()>;

    fn path(&self) -> PathBuf;
    fn get_disk_location(&self) -> DiskLocation;

    // Healing
    // DiskInfo
    // NSScanner

    // Volume operations.
    async fn make_volume(&self, volume: &str) -> Result<()>;
    async fn make_volumes(&self, volume: Vec<&str>) -> Result<()>;
    async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>;
    async fn stat_volume(&self, volume: &str) -> Result<VolumeInfo>;
    async fn delete_volume(&self, volume: &str) -> Result<()>;

    // 并发边读边写 w <- MetaCacheEntry
    async fn walk_dir<W: AsyncWrite + Unpin + Send>(&self, opts: WalkDirOptions, wr: &mut W) -> Result<()>;

    // Metadata operations
    async fn delete_version(
        &self,
        volume: &str,
        path: &str,
        fi: FileInfo,
        force_del_marker: bool,
        opts: DeleteOptions,
    ) -> Result<()>;
    async fn delete_versions(
        &self,
        volume: &str,
        versions: Vec<FileInfoVersions>,
        opts: DeleteOptions,
    ) -> Result<Vec<Option<Error>>>;
    async fn delete_paths(&self, volume: &str, paths: &[String]) -> Result<()>;
    async fn write_metadata(&self, org_volume: &str, volume: &str, path: &str, fi: FileInfo) -> Result<()>;
    async fn update_metadata(&self, volume: &str, path: &str, fi: FileInfo, opts: &UpdateMetadataOpts) -> Result<()>;
    async fn read_version(
        &self,
        org_volume: &str,
        volume: &str,
        path: &str,
        version_id: &str,
        opts: &ReadOptions,
    ) -> Result<FileInfo>;
    async fn read_xl(&self, volume: &str, path: &str, read_data: bool) -> Result<RawFileInfo>;
    async fn rename_data(
        &self,
        src_volume: &str,
        src_path: &str,
        file_info: FileInfo,
        dst_volume: &str,
        dst_path: &str,
    ) -> Result<RenameDataResp>;

    // File operations.
    // 读目录下的所有文件、目录
    async fn list_dir(&self, origvolume: &str, volume: &str, dir_path: &str, count: i32) -> Result<Vec<String>>;
    async fn read_file(&self, volume: &str, path: &str) -> Result<FileReader>;
    async fn read_file_stream(&self, volume: &str, path: &str, offset: usize, length: usize) -> Result<FileReader>;
    async fn append_file(&self, volume: &str, path: &str) -> Result<FileWriter>;
    async fn create_file(&self, origvolume: &str, volume: &str, path: &str, file_size: i64) -> Result<FileWriter>;
    // ReadFileStream
    async fn rename_file(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str) -> Result<()>;
    async fn rename_part(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str, meta: Bytes) -> Result<()>;
    async fn delete(&self, volume: &str, path: &str, opt: DeleteOptions) -> Result<()>;
    // VerifyFile
    async fn verify_file(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp>;
    // CheckParts
    async fn check_parts(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp>;
    // StatInfoFile
    // ReadParts
    async fn read_multiple(&self, req: ReadMultipleReq) -> Result<Vec<ReadMultipleResp>>;
    // CleanAbandonedData
    async fn write_all(&self, volume: &str, path: &str, data: Bytes) -> Result<()>;
    async fn read_all(&self, volume: &str, path: &str) -> Result<Bytes>;
    async fn disk_info(&self, opts: &DiskInfoOptions) -> Result<DiskInfo>;
    // async fn ns_scanner(
    //     &self,
    //     cache: &DataUsageCache,
    //     updates: Sender<DataUsageEntry>,
    //     scan_mode: HealScanMode,
    //     we_sleep: ShouldSleepFn,
    // ) -> Result<DataUsageCache>;
    // async fn healing(&self) -> Option<HealingTracker>;
}
