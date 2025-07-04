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

use std::collections::HashMap;
use std::fmt::Display;

use time::OffsetDateTime;

#[derive(Debug, Clone, Default)]
pub enum LcEventSrc {
    #[default]
    None,
    Heal,
    Scanner,
    Decom,
    Rebal,
    S3HeadObject,
    S3GetObject,
    S3ListObjects,
    S3PutObject,
    S3CopyObject,
    S3CompleteMultipartUpload,
}

#[derive(Clone, Debug, Default)]
pub struct LcAuditEvent {
    pub event: Event,
    pub source: LcEventSrc,
}

impl LcAuditEvent {
    pub fn new(event: Event, source: LcEventSrc) -> Self {
        Self { event, source }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExpirationOptions {
    pub expire: bool,
}

#[derive(Debug, Clone)]
pub struct Event {
    pub action: IlmAction,
    pub rule_id: String,
    pub due: Option<OffsetDateTime>,
    pub noncurrent_days: u32,
    pub newer_noncurrent_versions: usize,
    pub storage_class: String,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            action: IlmAction::NoneAction,
            rule_id: "".into(),
            due: Some(OffsetDateTime::UNIX_EPOCH),
            noncurrent_days: 0,
            newer_noncurrent_versions: 0,
            storage_class: "".into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IlmAction {
    NoneAction = 0,
    DeleteAction,
    DeleteVersionAction,
    TransitionAction,
    TransitionVersionAction,
    DeleteRestoredAction,
    DeleteRestoredVersionAction,
    DeleteAllVersionsAction,
    DelMarkerDeleteAllVersionsAction,
    ActionCount,
}

impl IlmAction {
    pub fn delete_restored(&self) -> bool {
        *self == Self::DeleteRestoredAction || *self == Self::DeleteRestoredVersionAction
    }

    pub fn delete_versioned(&self) -> bool {
        *self == Self::DeleteVersionAction || *self == Self::DeleteRestoredVersionAction
    }

    pub fn delete_all(&self) -> bool {
        *self == Self::DeleteAllVersionsAction || *self == Self::DelMarkerDeleteAllVersionsAction
    }

    pub fn delete(&self) -> bool {
        if self.delete_restored() {
            return true;
        }
        *self == Self::DeleteVersionAction
            || *self == Self::DeleteAction
            || *self == Self::DeleteAllVersionsAction
            || *self == Self::DelMarkerDeleteAllVersionsAction
    }
}

impl Display for IlmAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct TransitionedObject {
    pub name: String,
    pub version_id: String,
    pub tier: String,
    pub free_version: bool,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct TransitionOptions {
    pub status: String,
    pub tier: String,
    pub etag: String,
    pub restore_request: RestoreObjectRequest,
    pub restore_expiry: OffsetDateTime,
    pub expire_restored: bool,
}

impl Default for TransitionOptions {
    fn default() -> Self {
        Self {
            status: Default::default(),
            tier: Default::default(),
            etag: Default::default(),
            restore_request: Default::default(),
            restore_expiry: OffsetDateTime::now_utc(),
            expire_restored: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct S3Location {
    pub bucketname: String,
    //pub encryption:    Encryption,
    pub prefix: String,
    pub storage_class: String,
    //pub tagging:       Tags,
    pub user_metadata: HashMap<String, String>,
}

#[derive(Debug, Default, Clone)]
pub struct OutputLocation(pub S3Location);

#[derive(Debug, Default, Clone)]
pub struct RestoreObjectRequest {
    pub days: i64,
    pub ror_type: String,
    pub tier: String,
    pub description: String,
    //pub select_parameters: SelectParameters,
    pub output_location: OutputLocation,
}
