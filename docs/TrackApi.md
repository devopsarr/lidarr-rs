# \TrackApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_track_by_id**](TrackApi.md#get_track_by_id) | **GET** /api/v1/track/{id} | 
[**list_track**](TrackApi.md#list_track) | **GET** /api/v1/track | 



## get_track_by_id

> models::TrackResource get_track_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TrackResource**](TrackResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_track

> Vec<models::TrackResource> list_track(artist_id, album_id, album_release_id, track_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_id** | Option<**i32**> |  |  |
**album_release_id** | Option<**i32**> |  |  |
**track_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<models::TrackResource>**](TrackResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

