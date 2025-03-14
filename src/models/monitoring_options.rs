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
pub struct MonitoringOptions {
    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<models::MonitorTypes>,
    #[serde(rename = "albumsToMonitor", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub albums_to_monitor: Option<Option<Vec<String>>>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
}

impl MonitoringOptions {
    pub fn new() -> MonitoringOptions {
        MonitoringOptions {
            monitor: None,
            albums_to_monitor: None,
            monitored: None,
        }
    }
}

