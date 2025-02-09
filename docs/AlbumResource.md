# AlbumResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> |  | [optional]
**title** | Option<**String**> |  | [optional]
**disambiguation** | Option<**String**> |  | [optional]
**overview** | Option<**String**> |  | [optional]
**artist_id** | Option<**i32**> |  | [optional]
**foreign_album_id** | Option<**String**> |  | [optional]
**monitored** | Option<**bool**> |  | [optional]
**any_release_ok** | Option<**bool**> |  | [optional]
**profile_id** | Option<**i32**> |  | [optional]
**duration** | Option<**i32**> |  | [optional]
**album_type** | Option<**String**> |  | [optional]
**secondary_types** | Option<**Vec<String>**> |  | [optional]
**medium_count** | Option<**i32**> |  | [optional][readonly]
**ratings** | Option<[**models::Ratings**](Ratings.md)> |  | [optional]
**release_date** | Option<**String**> |  | [optional]
**releases** | Option<[**Vec<models::AlbumReleaseResource>**](AlbumReleaseResource.md)> |  | [optional]
**genres** | Option<**Vec<String>**> |  | [optional]
**media** | Option<[**Vec<models::MediumResource>**](MediumResource.md)> |  | [optional]
**artist** | Option<[**models::ArtistResource**](ArtistResource.md)> |  | [optional]
**images** | Option<[**Vec<models::MediaCover>**](MediaCover.md)> |  | [optional]
**links** | Option<[**Vec<models::Links>**](Links.md)> |  | [optional]
**last_search_time** | Option<**String**> |  | [optional]
**statistics** | Option<[**models::AlbumStatisticsResource**](AlbumStatisticsResource.md)> |  | [optional]
**add_options** | Option<[**models::AddAlbumOptions**](AddAlbumOptions.md)> |  | [optional]
**remote_cover** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


