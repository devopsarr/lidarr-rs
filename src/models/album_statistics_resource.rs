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
pub struct AlbumStatisticsResource {
    #[serde(rename = "trackFileCount", skip_serializing_if = "Option::is_none")]
    pub track_file_count: Option<i32>,
    #[serde(rename = "trackCount", skip_serializing_if = "Option::is_none")]
    pub track_count: Option<i32>,
    #[serde(rename = "totalTrackCount", skip_serializing_if = "Option::is_none")]
    pub total_track_count: Option<i32>,
    #[serde(rename = "sizeOnDisk", skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,
    #[serde(rename = "percentOfTracks", skip_serializing_if = "Option::is_none")]
    pub percent_of_tracks: Option<f64>,
}

impl AlbumStatisticsResource {
    pub fn new() -> AlbumStatisticsResource {
        AlbumStatisticsResource {
            track_file_count: None,
            track_count: None,
            total_track_count: None,
            size_on_disk: None,
            percent_of_tracks: None,
        }
    }
}

