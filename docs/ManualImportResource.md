# ManualImportResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**size** | Option<**i64**> |  | [optional]
**artist** | Option<[**models::ArtistResource**](ArtistResource.md)> |  | [optional]
**album** | Option<[**models::AlbumResource**](AlbumResource.md)> |  | [optional]
**album_release_id** | Option<**i32**> |  | [optional]
**tracks** | Option<[**Vec<models::TrackResource>**](TrackResource.md)> |  | [optional]
**quality** | Option<[**models::QualityModel**](QualityModel.md)> |  | [optional]
**release_group** | Option<**String**> |  | [optional]
**quality_weight** | Option<**i32**> |  | [optional]
**download_id** | Option<**String**> |  | [optional]
**indexer_flags** | Option<**i32**> |  | [optional]
**rejections** | Option<[**Vec<models::Rejection>**](Rejection.md)> |  | [optional]
**audio_tags** | Option<[**models::ParsedTrackInfo**](ParsedTrackInfo.md)> |  | [optional]
**additional_file** | Option<**bool**> |  | [optional]
**replace_existing_files** | Option<**bool**> |  | [optional]
**disable_release_switching** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


