# QueueResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**artist_id** | Option<**i32**> |  | [optional]
**album_id** | Option<**i32**> |  | [optional]
**artist** | Option<[**models::ArtistResource**](ArtistResource.md)> |  | [optional]
**album** | Option<[**models::AlbumResource**](AlbumResource.md)> |  | [optional]
**quality** | Option<[**models::QualityModel**](QualityModel.md)> |  | [optional]
**custom_formats** | Option<[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)> |  | [optional]
**custom_format_score** | Option<**i32**> |  | [optional]
**size** | Option<**f64**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**sizeleft** | Option<**f64**> |  | [optional]
**timeleft** | Option<**String**> |  | [optional]
**estimated_completion_time** | Option<**String**> |  | [optional]
**added** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**tracked_download_status** | Option<[**models::TrackedDownloadStatus**](TrackedDownloadStatus.md)> |  | [optional]
**tracked_download_state** | Option<[**models::TrackedDownloadState**](TrackedDownloadState.md)> |  | [optional]
**status_messages** | Option<[**Vec<models::TrackedDownloadStatusMessage>**](TrackedDownloadStatusMessage.md)> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**download_id** | Option<**String**> |  | [optional]
**protocol** | Option<[**models::DownloadProtocol**](DownloadProtocol.md)> |  | [optional]
**download_client** | Option<**String**> |  | [optional]
**download_client_has_post_import_category** | Option<**bool**> |  | [optional]
**indexer** | Option<**String**> |  | [optional]
**output_path** | Option<**String**> |  | [optional]
**track_file_count** | Option<**i32**> |  | [optional]
**track_has_file_count** | Option<**i32**> |  | [optional]
**download_forced** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


