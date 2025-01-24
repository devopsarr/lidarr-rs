# \HistoryApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_history_failed_by_id**](HistoryApi.md#create_history_failed_by_id) | **POST** /api/v1/history/failed/{id} | 
[**get_history**](HistoryApi.md#get_history) | **GET** /api/v1/history | 
[**list_history_artist**](HistoryApi.md#list_history_artist) | **GET** /api/v1/history/artist | 
[**list_history_since**](HistoryApi.md#list_history_since) | **GET** /api/v1/history/since | 



## create_history_failed_by_id

> create_history_failed_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history

> models::HistoryResourcePagingResource get_history(page, page_size, sort_key, sort_direction, include_artist, include_album, include_track, event_type, album_id, download_id, artist_ids, quality)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_artist** | Option<**bool**> |  |  |
**include_album** | Option<**bool**> |  |  |
**include_track** | Option<**bool**> |  |  |
**event_type** | Option<[**Vec<i32>**](i32.md)> |  |  |
**album_id** | Option<**i32**> |  |  |
**download_id** | Option<**String**> |  |  |
**artist_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**quality** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::HistoryResourcePagingResource**](HistoryResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_artist

> Vec<models::HistoryResource> list_history_artist(artist_id, album_id, event_type, include_artist, include_album, include_track)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_id** | Option<**i32**> |  |  |
**event_type** | Option<[**EntityHistoryEventType**](.md)> |  |  |
**include_artist** | Option<**bool**> |  |  |[default to false]
**include_album** | Option<**bool**> |  |  |[default to false]
**include_track** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_since

> Vec<models::HistoryResource> list_history_since(date, event_type, include_artist, include_album, include_track)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> |  |  |
**event_type** | Option<[**EntityHistoryEventType**](.md)> |  |  |
**include_artist** | Option<**bool**> |  |  |[default to false]
**include_album** | Option<**bool**> |  |  |[default to false]
**include_track** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

