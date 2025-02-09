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
pub struct UpdateChanges {
    #[serde(rename = "new", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new: Option<Option<Vec<String>>>,
    #[serde(rename = "fixed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Option<Vec<String>>>,
}

impl UpdateChanges {
    pub fn new() -> UpdateChanges {
        UpdateChanges {
            new: None,
            fixed: None,
        }
    }
}

