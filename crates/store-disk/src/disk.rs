use bytes::Bytes;
use rustfs_disk_core::error::{Error, Result};
use rustfs_disk_core::{DiskAPI, FileWriter};
use rustfs_disk_core::{FileReader, types::*};
use rustfs_disk_local::local::LocalDisk;
use rustfs_disk_remote::remote::RemoteDisk;
use rustfs_filemeta::FileInfo;
use rustfs_filemeta::FileInfoVersions;
use rustfs_filemeta::RawFileInfo;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

pub type DiskStore = Arc<Disk>;

pub async fn new_disk(ep: &rustfs_endpoints::Endpoint, opt: &DiskOption) -> Result<DiskStore> {
    if ep.is_local {
        let s = LocalDisk::new(ep, opt.cleanup).await?;
        Ok(Arc::new(Disk::Local(s)))
    } else {
        let remote_disk = RemoteDisk::new(ep, opt).await?;
        Ok(Arc::new(Disk::Remote(remote_disk)))
    }
}

#[derive(Debug)]
pub enum Disk {
    Local(LocalDisk),
    Remote(RemoteDisk),
}

#[async_trait::async_trait]
impl DiskAPI for Disk {
    #[tracing::instrument(skip(self))]
    fn to_string(&self) -> String {
        match self {
            Disk::Local(local_disk) => local_disk.to_string(),
            Disk::Remote(remote_disk) => remote_disk.to_string(),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn is_online(&self) -> bool {
        match self {
            Disk::Local(local_disk) => local_disk.is_online().await,
            Disk::Remote(remote_disk) => remote_disk.is_online().await,
        }
    }

    #[tracing::instrument(skip(self))]
    fn is_local(&self) -> bool {
        match self {
            Disk::Local(local_disk) => local_disk.is_local(),
            Disk::Remote(remote_disk) => remote_disk.is_local(),
        }
    }

    #[tracing::instrument(skip(self))]
    fn host_name(&self) -> String {
        match self {
            Disk::Local(local_disk) => local_disk.host_name(),
            Disk::Remote(remote_disk) => remote_disk.host_name(),
        }
    }

    #[tracing::instrument(skip(self))]
    fn endpoint(&self) -> rustfs_endpoints::Endpoint {
        match self {
            Disk::Local(local_disk) => local_disk.endpoint(),
            Disk::Remote(remote_disk) => remote_disk.endpoint(),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn close(&self) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.close().await,
            Disk::Remote(remote_disk) => remote_disk.close().await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_disk_id(&self) -> Result<Option<Uuid>> {
        match self {
            Disk::Local(local_disk) => local_disk.get_disk_id().await,
            Disk::Remote(remote_disk) => remote_disk.get_disk_id().await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn set_disk_id(&self, id: Option<Uuid>) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.set_disk_id(id).await,
            Disk::Remote(remote_disk) => remote_disk.set_disk_id(id).await,
        }
    }

    #[tracing::instrument(skip(self))]
    fn path(&self) -> PathBuf {
        match self {
            Disk::Local(local_disk) => local_disk.path(),
            Disk::Remote(remote_disk) => remote_disk.path(),
        }
    }

    #[tracing::instrument(skip(self))]
    fn get_disk_location(&self) -> DiskLocation {
        match self {
            Disk::Local(local_disk) => local_disk.get_disk_location(),
            Disk::Remote(remote_disk) => remote_disk.get_disk_location(),
        }
    }

    #[tracing::instrument(skip(self))]
    async fn make_volume(&self, volume: &str) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.make_volume(volume).await,
            Disk::Remote(remote_disk) => remote_disk.make_volume(volume).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn make_volumes(&self, volumes: Vec<&str>) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.make_volumes(volumes).await,
            Disk::Remote(remote_disk) => remote_disk.make_volumes(volumes).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn list_volumes(&self) -> Result<Vec<VolumeInfo>> {
        match self {
            Disk::Local(local_disk) => local_disk.list_volumes().await,
            Disk::Remote(remote_disk) => remote_disk.list_volumes().await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn stat_volume(&self, volume: &str) -> Result<VolumeInfo> {
        match self {
            Disk::Local(local_disk) => local_disk.stat_volume(volume).await,
            Disk::Remote(remote_disk) => remote_disk.stat_volume(volume).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_volume(&self, volume: &str) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.delete_volume(volume).await,
            Disk::Remote(remote_disk) => remote_disk.delete_volume(volume).await,
        }
    }

    #[tracing::instrument(skip(self, wr))]
    async fn walk_dir(&self, opts: WalkDirOptions, wr: &mut FileWriter) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.walk_dir(opts, wr).await,
            Disk::Remote(remote_disk) => remote_disk.walk_dir(opts, wr).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_version(
        &self,
        volume: &str,
        path: &str,
        fi: FileInfo,
        force_del_marker: bool,
        opts: DeleteOptions,
    ) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.delete_version(volume, path, fi, force_del_marker, opts).await,
            Disk::Remote(remote_disk) => remote_disk.delete_version(volume, path, fi, force_del_marker, opts).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_versions(
        &self,
        volume: &str,
        versions: Vec<FileInfoVersions>,
        opts: DeleteOptions,
    ) -> Result<Vec<Option<Error>>> {
        match self {
            Disk::Local(local_disk) => local_disk.delete_versions(volume, versions, opts).await,
            Disk::Remote(remote_disk) => remote_disk.delete_versions(volume, versions, opts).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete_paths(&self, volume: &str, paths: &[String]) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.delete_paths(volume, paths).await,
            Disk::Remote(remote_disk) => remote_disk.delete_paths(volume, paths).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn write_metadata(&self, _org_volume: &str, volume: &str, path: &str, fi: FileInfo) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.write_metadata(_org_volume, volume, path, fi).await,
            Disk::Remote(remote_disk) => remote_disk.write_metadata(_org_volume, volume, path, fi).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn update_metadata(&self, volume: &str, path: &str, fi: FileInfo, opts: &UpdateMetadataOpts) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.update_metadata(volume, path, fi, opts).await,
            Disk::Remote(remote_disk) => remote_disk.update_metadata(volume, path, fi, opts).await,
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn read_version(
        &self,
        _org_volume: &str,
        volume: &str,
        path: &str,
        version_id: &str,
        opts: &ReadOptions,
    ) -> Result<FileInfo> {
        match self {
            Disk::Local(local_disk) => local_disk.read_version(_org_volume, volume, path, version_id, opts).await,
            Disk::Remote(remote_disk) => remote_disk.read_version(_org_volume, volume, path, version_id, opts).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn read_xl(&self, volume: &str, path: &str, read_data: bool) -> Result<RawFileInfo> {
        match self {
            Disk::Local(local_disk) => local_disk.read_xl(volume, path, read_data).await,
            Disk::Remote(remote_disk) => remote_disk.read_xl(volume, path, read_data).await,
        }
    }

    #[tracing::instrument(skip(self, fi))]
    async fn rename_data(
        &self,
        src_volume: &str,
        src_path: &str,
        fi: FileInfo,
        dst_volume: &str,
        dst_path: &str,
    ) -> Result<RenameDataResp> {
        match self {
            Disk::Local(local_disk) => local_disk.rename_data(src_volume, src_path, fi, dst_volume, dst_path).await,
            Disk::Remote(remote_disk) => remote_disk.rename_data(src_volume, src_path, fi, dst_volume, dst_path).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn list_dir(&self, _origvolume: &str, volume: &str, _dir_path: &str, _count: i32) -> Result<Vec<String>> {
        match self {
            Disk::Local(local_disk) => local_disk.list_dir(_origvolume, volume, _dir_path, _count).await,
            Disk::Remote(remote_disk) => remote_disk.list_dir(_origvolume, volume, _dir_path, _count).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn read_file(&self, volume: &str, path: &str) -> Result<FileReader> {
        match self {
            Disk::Local(local_disk) => local_disk.read_file(volume, path).await,
            Disk::Remote(remote_disk) => remote_disk.read_file(volume, path).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn read_file_stream(&self, volume: &str, path: &str, offset: usize, length: usize) -> Result<FileReader> {
        match self {
            Disk::Local(local_disk) => local_disk.read_file_stream(volume, path, offset, length).await,
            Disk::Remote(remote_disk) => remote_disk.read_file_stream(volume, path, offset, length).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn append_file(&self, volume: &str, path: &str) -> Result<FileWriter> {
        match self {
            Disk::Local(local_disk) => local_disk.append_file(volume, path).await,
            Disk::Remote(remote_disk) => remote_disk.append_file(volume, path).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn create_file(&self, _origvolume: &str, volume: &str, path: &str, _file_size: i64) -> Result<FileWriter> {
        match self {
            Disk::Local(local_disk) => local_disk.create_file(_origvolume, volume, path, _file_size).await,
            Disk::Remote(remote_disk) => remote_disk.create_file(_origvolume, volume, path, _file_size).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn rename_file(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.rename_file(src_volume, src_path, dst_volume, dst_path).await,
            Disk::Remote(remote_disk) => remote_disk.rename_file(src_volume, src_path, dst_volume, dst_path).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn rename_part(&self, src_volume: &str, src_path: &str, dst_volume: &str, dst_path: &str, meta: Bytes) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.rename_part(src_volume, src_path, dst_volume, dst_path, meta).await,
            Disk::Remote(remote_disk) => {
                remote_disk
                    .rename_part(src_volume, src_path, dst_volume, dst_path, meta)
                    .await
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, volume: &str, path: &str, opt: DeleteOptions) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.delete(volume, path, opt).await,
            Disk::Remote(remote_disk) => remote_disk.delete(volume, path, opt).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn verify_file(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp> {
        match self {
            Disk::Local(local_disk) => local_disk.verify_file(volume, path, fi).await,
            Disk::Remote(remote_disk) => remote_disk.verify_file(volume, path, fi).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn check_parts(&self, volume: &str, path: &str, fi: &FileInfo) -> Result<CheckPartsResp> {
        match self {
            Disk::Local(local_disk) => local_disk.check_parts(volume, path, fi).await,
            Disk::Remote(remote_disk) => remote_disk.check_parts(volume, path, fi).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn read_multiple(&self, req: ReadMultipleReq) -> Result<Vec<ReadMultipleResp>> {
        match self {
            Disk::Local(local_disk) => local_disk.read_multiple(req).await,
            Disk::Remote(remote_disk) => remote_disk.read_multiple(req).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn write_all(&self, volume: &str, path: &str, data: Bytes) -> Result<()> {
        match self {
            Disk::Local(local_disk) => local_disk.write_all(volume, path, data).await,
            Disk::Remote(remote_disk) => remote_disk.write_all(volume, path, data).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn read_all(&self, volume: &str, path: &str) -> Result<Bytes> {
        match self {
            Disk::Local(local_disk) => local_disk.read_all(volume, path).await,
            Disk::Remote(remote_disk) => remote_disk.read_all(volume, path).await,
        }
    }

    #[tracing::instrument(skip(self))]
    async fn disk_info(&self, opts: &DiskInfoOptions) -> Result<DiskInfo> {
        match self {
            Disk::Local(local_disk) => local_disk.disk_info(opts).await,
            Disk::Remote(remote_disk) => remote_disk.disk_info(opts).await,
        }
    }

    // #[tracing::instrument(skip(self, cache, we_sleep, scan_mode))]
    // async fn ns_scanner(
    //     &self,
    //     cache: &DataUsageCache,
    //     updates: Sender<DataUsageEntry>,
    //     scan_mode: HealScanMode,
    //     we_sleep: ShouldSleepFn,
    // ) -> Result<DataUsageCache> {
    //     match self {
    //         Disk::Local(local_disk) => local_disk.ns_scanner(cache, updates, scan_mode, we_sleep).await,
    //         Disk::Remote(remote_disk) => remote_disk.ns_scanner(cache, updates, scan_mode, we_sleep).await,
    //     }
    // }

    #[tracing::instrument(skip(self))]
    async fn healing(&self) -> Option<Bytes> {
        match self {
            Disk::Local(local_disk) => local_disk.healing().await,
            Disk::Remote(remote_disk) => remote_disk.healing().await,
        }
    }
}
