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

use crate::heal::{
    data_scanner::ShouldSleepFn,
    data_usage_cache::{DataUsageCache, DataUsageEntry},
    heal_commands::{HealScanMode, HealingTracker},
};
use rustfs_disk_core::error::Result;
use std::fmt::Debug;
use tokio::sync::mpsc::Sender;

#[async_trait::async_trait]
pub trait ScannerAPI: Debug + Send + Sync + 'static {
    async fn ns_scanner(
        &self,
        cache: &DataUsageCache,
        updates: Sender<DataUsageEntry>,
        scan_mode: HealScanMode,
        we_sleep: ShouldSleepFn,
    ) -> Result<DataUsageCache>;
    async fn healing(&self) -> Option<HealingTracker>;
}
