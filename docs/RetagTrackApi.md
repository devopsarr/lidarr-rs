# \RetagTrackApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_retag**](RetagTrackApi.md#list_retag) | **GET** /api/v1/retag | 



## list_retag

> Vec<models::RetagTrackResource> list_retag(artist_id, album_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::RetagTrackResource>**](RetagTrackResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

