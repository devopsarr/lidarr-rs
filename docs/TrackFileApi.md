# \TrackFileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_track_file**](TrackFileApi.md#delete_track_file) | **DELETE** /api/v1/trackfile/{id} | 
[**delete_track_file_bulk**](TrackFileApi.md#delete_track_file_bulk) | **DELETE** /api/v1/trackfile/bulk | 
[**get_track_file_by_id**](TrackFileApi.md#get_track_file_by_id) | **GET** /api/v1/trackfile/{id} | 
[**list_track_file**](TrackFileApi.md#list_track_file) | **GET** /api/v1/trackfile | 
[**put_track_file_editor**](TrackFileApi.md#put_track_file_editor) | **PUT** /api/v1/trackfile/editor | 
[**update_track_file**](TrackFileApi.md#update_track_file) | **PUT** /api/v1/trackfile/{id} | 



## delete_track_file

> delete_track_file(id)


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


## delete_track_file_bulk

> delete_track_file_bulk(track_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_file_list_resource** | Option<[**TrackFileListResource**](TrackFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_track_file_by_id

> models::TrackFileResource get_track_file_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TrackFileResource**](TrackFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_track_file

> Vec<models::TrackFileResource> list_track_file(artist_id, track_file_ids, album_id, unmapped)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**track_file_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**album_id** | Option<[**Vec<i32>**](i32.md)> |  |  |
**unmapped** | Option<**bool**> |  |  |

### Return type

[**Vec<models::TrackFileResource>**](TrackFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_track_file_editor

> put_track_file_editor(track_file_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**track_file_list_resource** | Option<[**TrackFileListResource**](TrackFileListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_track_file

> models::TrackFileResource update_track_file(id, track_file_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**track_file_resource** | Option<[**TrackFileResource**](TrackFileResource.md)> |  |  |

### Return type

[**models::TrackFileResource**](TrackFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

