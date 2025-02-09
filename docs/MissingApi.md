# \MissingApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_wanted_missing**](MissingApi.md#get_wanted_missing) | **GET** /api/v1/wanted/missing | 
[**get_wanted_missing_by_id**](MissingApi.md#get_wanted_missing_by_id) | **GET** /api/v1/wanted/missing/{id} | 



## get_wanted_missing

> models::AlbumResourcePagingResource get_wanted_missing(page, page_size, sort_key, sort_direction, include_artist, monitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**include_artist** | Option<**bool**> |  |  |[default to false]
**monitored** | Option<**bool**> |  |  |[default to true]

### Return type

[**models::AlbumResourcePagingResource**](AlbumResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wanted_missing_by_id

> models::AlbumResource get_wanted_missing_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AlbumResource**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

