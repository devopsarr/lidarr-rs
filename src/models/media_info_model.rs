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
pub struct MediaInfoModel {
    #[serde(rename = "audioFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub audio_format: Option<Option<String>>,
    #[serde(rename = "audioBitrate", skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<i32>,
    #[serde(rename = "audioChannels", skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i32>,
    #[serde(rename = "audioBits", skip_serializing_if = "Option::is_none")]
    pub audio_bits: Option<i32>,
    #[serde(rename = "audioSampleRate", skip_serializing_if = "Option::is_none")]
    pub audio_sample_rate: Option<i32>,
}

impl MediaInfoModel {
    pub fn new() -> MediaInfoModel {
        MediaInfoModel {
            audio_format: None,
            audio_bitrate: None,
            audio_channels: None,
            audio_bits: None,
            audio_sample_rate: None,
        }
    }
}

