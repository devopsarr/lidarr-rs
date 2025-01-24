# TrackFileResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**artist_id** | Option<**i32**> |  | [optional]
**album_id** | Option<**i32**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**date_added** | Option<**String**> |  | [optional]
**scene_name** | Option<**String**> |  | [optional]
**release_group** | Option<**String**> |  | [optional]
**quality** | Option<[**models::QualityModel**](QualityModel.md)> |  | [optional]
**quality_weight** | Option<**i32**> |  | [optional]
**custom_formats** | Option<[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)> |  | [optional]
**custom_format_score** | Option<**i32**> |  | [optional]
**indexer_flags** | Option<**i32**> |  | [optional]
**media_info** | Option<[**models::MediaInfoResource**](MediaInfoResource.md)> |  | [optional]
**quality_cutoff_not_met** | Option<**bool**> |  | [optional]
**audio_tags** | Option<[**models::ParsedTrackInfo**](ParsedTrackInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


