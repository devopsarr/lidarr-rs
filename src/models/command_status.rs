/*
 * Lidarr
 *
 * Lidarr API docs
 *
 * The version of the OpenAPI document: v2.9.6.4552
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommandStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "orphaned")]
    Orphaned,

}

impl std::fmt::Display for CommandStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::Started => write!(f, "started"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Aborted => write!(f, "aborted"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Orphaned => write!(f, "orphaned"),
        }
    }
}

impl Default for CommandStatus {
    fn default() -> CommandStatus {
        Self::Queued
    }
}

