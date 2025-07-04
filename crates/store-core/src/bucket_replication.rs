use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplicationStatusType {
    #[default]
    Pending,
    Completed,
    CompletedLegacy,
    Failed,
    Replica,
    Unknown,
}

impl ReplicationStatusType {
    // Converts the enum variant to its string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ReplicationStatusType::Pending => "PENDING",
            ReplicationStatusType::Completed => "COMPLETED",
            ReplicationStatusType::CompletedLegacy => "COMPLETE",
            ReplicationStatusType::Failed => "FAILED",
            ReplicationStatusType::Replica => "REPLICA",
            ReplicationStatusType::Unknown => "",
        }
    }

    // Checks if the status is empty (not set)
    pub fn is_empty(&self) -> bool {
        matches!(self, ReplicationStatusType::Pending) // Adjust logic if needed
    }

    // 从字符串构造 ReplicationStatusType 枚举
    pub fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "PENDING" => ReplicationStatusType::Pending,
            "COMPLETED" => ReplicationStatusType::Completed,
            "COMPLETE" => ReplicationStatusType::CompletedLegacy,
            "FAILED" => ReplicationStatusType::Failed,
            "REPLICA" => ReplicationStatusType::Replica,
            other => ReplicationStatusType::Unknown,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionPurgeStatusType {
    Pending,
    Complete,
    Failed,
    Empty,
    #[default]
    Unknown,
}

impl VersionPurgeStatusType {
    // 检查是否是 Empty
    pub fn is_empty(&self) -> bool {
        matches!(self, VersionPurgeStatusType::Empty)
    }

    // 检查是否是 Pending（Pending 或 Failed 都算作 Pending 状态）
    pub fn is_pending(&self) -> bool {
        matches!(self, VersionPurgeStatusType::Pending | VersionPurgeStatusType::Failed)
    }
}

// 从字符串实现转换（类似于 Go 的字符串比较）
impl From<&str> for VersionPurgeStatusType {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "PENDING" => VersionPurgeStatusType::Pending,
            "COMPLETE" => VersionPurgeStatusType::Complete,
            "FAILED" => VersionPurgeStatusType::Failed,
            _ => VersionPurgeStatusType::Empty,
        }
    }
}

impl fmt::Display for VersionPurgeStatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VersionPurgeStatusType::Pending => "PENDING",
            VersionPurgeStatusType::Complete => "COMPLETE",
            VersionPurgeStatusType::Failed => "FAILED",
            VersionPurgeStatusType::Empty => "",
            VersionPurgeStatusType::Unknown => "UNKNOWN",
        };
        write!(f, "{s}")
    }
}

pub fn get_composite_version_purge_status(status_map: &HashMap<String, VersionPurgeStatusType>) -> VersionPurgeStatusType {
    if status_map.is_empty() {
        return VersionPurgeStatusType::Unknown;
    }

    let mut completed_count = 0;

    for status in status_map.values() {
        match status {
            VersionPurgeStatusType::Failed => return VersionPurgeStatusType::Failed,
            VersionPurgeStatusType::Complete => completed_count += 1,
            _ => {}
        }
    }

    if completed_count == status_map.len() {
        VersionPurgeStatusType::Complete
    } else {
        VersionPurgeStatusType::Pending
    }
}
