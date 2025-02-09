# HistoryResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**album_id** | Option<**i32**> |  | [optional]
**artist_id** | Option<**i32**> |  | [optional]
**track_id** | Option<**i32**> |  | [optional]
**source_title** | Option<**String**> |  | [optional]
**quality** | Option<[**models::QualityModel**](QualityModel.md)> |  | [optional]
**custom_formats** | Option<[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)> |  | [optional]
**custom_format_score** | Option<**i32**> |  | [optional]
**quality_cutoff_not_met** | Option<**bool**> |  | [optional]
**date** | Option<**String**> |  | [optional]
**download_id** | Option<**String**> |  | [optional]
**event_type** | Option<[**models::EntityHistoryEventType**](EntityHistoryEventType.md)> |  | [optional]
**data** | Option<**std::collections::HashMap<String, String>**> |  | [optional]
**album** | Option<[**models::AlbumResource**](AlbumResource.md)> |  | [optional]
**artist** | Option<[**models::ArtistResource**](ArtistResource.md)> |  | [optional]
**track** | Option<[**models::TrackResource**](TrackResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


