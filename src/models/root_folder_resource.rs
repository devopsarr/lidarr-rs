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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RootFolderResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "defaultMetadataProfileId", skip_serializing_if = "Option::is_none")]
    pub default_metadata_profile_id: Option<i32>,
    #[serde(rename = "defaultQualityProfileId", skip_serializing_if = "Option::is_none")]
    pub default_quality_profile_id: Option<i32>,
    #[serde(rename = "defaultMonitorOption", skip_serializing_if = "Option::is_none")]
    pub default_monitor_option: Option<models::MonitorTypes>,
    #[serde(rename = "defaultNewItemMonitorOption", skip_serializing_if = "Option::is_none")]
    pub default_new_item_monitor_option: Option<models::NewItemMonitorTypes>,
    #[serde(rename = "defaultTags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub default_tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "accessible", skip_serializing_if = "Option::is_none")]
    pub accessible: Option<bool>,
    #[serde(rename = "freeSpace", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub free_space: Option<Option<i64>>,
    #[serde(rename = "totalSpace", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub total_space: Option<Option<i64>>,
}

impl RootFolderResource {
    pub fn new() -> RootFolderResource {
        RootFolderResource {
            id: None,
            name: None,
            path: None,
            default_metadata_profile_id: None,
            default_quality_profile_id: None,
            default_monitor_option: None,
            default_new_item_monitor_option: None,
            default_tags: None,
            accessible: None,
            free_space: None,
            total_space: None,
        }
    }
}

