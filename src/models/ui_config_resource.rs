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
pub struct UiConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "firstDayOfWeek", skip_serializing_if = "Option::is_none")]
    pub first_day_of_week: Option<i32>,
    #[serde(rename = "calendarWeekColumnHeader", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub calendar_week_column_header: Option<Option<String>>,
    #[serde(rename = "shortDateFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub short_date_format: Option<Option<String>>,
    #[serde(rename = "longDateFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub long_date_format: Option<Option<String>>,
    #[serde(rename = "timeFormat", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_format: Option<Option<String>>,
    #[serde(rename = "showRelativeDates", skip_serializing_if = "Option::is_none")]
    pub show_relative_dates: Option<bool>,
    #[serde(rename = "enableColorImpairedMode", skip_serializing_if = "Option::is_none")]
    pub enable_color_impaired_mode: Option<bool>,
    #[serde(rename = "uiLanguage", skip_serializing_if = "Option::is_none")]
    pub ui_language: Option<i32>,
    #[serde(rename = "expandAlbumByDefault", skip_serializing_if = "Option::is_none")]
    pub expand_album_by_default: Option<bool>,
    #[serde(rename = "expandSingleByDefault", skip_serializing_if = "Option::is_none")]
    pub expand_single_by_default: Option<bool>,
    #[serde(rename = "expandEPByDefault", skip_serializing_if = "Option::is_none")]
    pub expand_epby_default: Option<bool>,
    #[serde(rename = "expandBroadcastByDefault", skip_serializing_if = "Option::is_none")]
    pub expand_broadcast_by_default: Option<bool>,
    #[serde(rename = "expandOtherByDefault", skip_serializing_if = "Option::is_none")]
    pub expand_other_by_default: Option<bool>,
    #[serde(rename = "theme", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub theme: Option<Option<String>>,
}

impl UiConfigResource {
    pub fn new() -> UiConfigResource {
        UiConfigResource {
            id: None,
            first_day_of_week: None,
            calendar_week_column_header: None,
            short_date_format: None,
            long_date_format: None,
            time_format: None,
            show_relative_dates: None,
            enable_color_impaired_mode: None,
            ui_language: None,
            expand_album_by_default: None,
            expand_single_by_default: None,
            expand_epby_default: None,
            expand_broadcast_by_default: None,
            expand_other_by_default: None,
            theme: None,
        }
    }
}

