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
pub struct IsoCountry {
    #[serde(rename = "twoLetterCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub two_letter_code: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
}

impl IsoCountry {
    pub fn new() -> IsoCountry {
        IsoCountry {
            two_letter_code: None,
            name: None,
        }
    }
}

