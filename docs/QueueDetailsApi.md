# \QueueDetailsApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_queue_details**](QueueDetailsApi.md#list_queue_details) | **GET** /api/v1/queue/details | 



## list_queue_details

> Vec<models::QueueResource> list_queue_details(artist_id, album_ids, include_artist, include_album)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**include_artist** | Option<**bool**> |  |  |[default to false]
**include_album** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::QueueResource>**](QueueResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

