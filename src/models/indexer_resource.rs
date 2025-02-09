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
pub struct IndexerResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "fields", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Option<Vec<models::Field>>>,
    #[serde(rename = "implementationName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation_name: Option<Option<String>>,
    #[serde(rename = "implementation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub implementation: Option<Option<String>>,
    #[serde(rename = "configContract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub config_contract: Option<Option<String>>,
    #[serde(rename = "infoLink", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub info_link: Option<Option<String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<models::ProviderMessage>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "presets", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presets: Option<Option<Vec<models::IndexerResource>>>,
    #[serde(rename = "enableRss", skip_serializing_if = "Option::is_none")]
    pub enable_rss: Option<bool>,
    #[serde(rename = "enableAutomaticSearch", skip_serializing_if = "Option::is_none")]
    pub enable_automatic_search: Option<bool>,
    #[serde(rename = "enableInteractiveSearch", skip_serializing_if = "Option::is_none")]
    pub enable_interactive_search: Option<bool>,
    #[serde(rename = "supportsRss", skip_serializing_if = "Option::is_none")]
    pub supports_rss: Option<bool>,
    #[serde(rename = "supportsSearch", skip_serializing_if = "Option::is_none")]
    pub supports_search: Option<bool>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<models::DownloadProtocol>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "downloadClientId", skip_serializing_if = "Option::is_none")]
    pub download_client_id: Option<i32>,
}

impl IndexerResource {
    pub fn new() -> IndexerResource {
        IndexerResource {
            id: None,
            name: None,
            fields: None,
            implementation_name: None,
            implementation: None,
            config_contract: None,
            info_link: None,
            message: None,
            tags: None,
            presets: None,
            enable_rss: None,
            enable_automatic_search: None,
            enable_interactive_search: None,
            supports_rss: None,
            supports_search: None,
            protocol: None,
            priority: None,
            download_client_id: None,
        }
    }
}

