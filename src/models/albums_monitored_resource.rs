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
pub struct AlbumsMonitoredResource {
    #[serde(rename = "albumIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub album_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
}

impl AlbumsMonitoredResource {
    pub fn new() -> AlbumsMonitoredResource {
        AlbumsMonitoredResource {
            album_ids: None,
            monitored: None,
        }
    }
}

