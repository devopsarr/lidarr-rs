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
pub enum NewItemMonitorTypes {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "new")]
    New,

}

impl std::fmt::Display for NewItemMonitorTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::New => write!(f, "new"),
        }
    }
}

impl Default for NewItemMonitorTypes {
    fn default() -> NewItemMonitorTypes {
        Self::All
    }
}

