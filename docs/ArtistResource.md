# ArtistResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**status** | Option<[**models::ArtistStatusType**](ArtistStatusType.md)> |  | [optional]
**ended** | Option<**bool**> |  | [optional][readonly]
**artist_name** | Option<**String**> |  | [optional]
**foreign_artist_id** | Option<**String**> |  | [optional]
**mb_id** | Option<**String**> |  | [optional]
**tadb_id** | Option<**i32**> |  | [optional]
**discogs_id** | Option<**i32**> |  | [optional]
**all_music_id** | Option<**String**> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**artist_type** | Option<**String**> |  | [optional]
**disambiguation** | Option<**String**> |  | [optional]
**links** | Option<[**Vec<models::Links>**](Links.md)> |  | [optional]
**next_album** | Option<[**models::AlbumResource**](AlbumResource.md)> |  | [optional]
**last_album** | Option<[**models::AlbumResource**](AlbumResource.md)> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**members** | Option<[**Vec<models::Member>**](Member.md)> |  | [optional]
**remote_poster** | Option<**String**> |  | [optional]
**path** | Option<**String**> |  | [optional]
**quality_profile_id** | Option<**i32**> |  | [optional]
**metadata_profile_id** | Option<**i32**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**monitor_new_items** | Option<[**models::NewItemMonitorTypes**](NewItemMonitorTypes.md)> |  | [optional]
**root_folder_path** | Option<**String**> |  | [optional]
**folder** | Option<**String**> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**clean_name** | Option<**String**> |  | [optional]
**sort_name** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<i32>**> |  | [optional]
**added** | Option<**String**> |  | [optional]
**add_options** | Option<[**models::AddArtistOptions**](AddArtistOptions.md)> |  | [optional]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**statistics** | Option<[**models::ArtistStatisticsResource**](ArtistStatisticsResource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


