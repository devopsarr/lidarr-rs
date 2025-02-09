# \RenameTrackApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_rename**](RenameTrackApi.md#list_rename) | **GET** /api/v1/rename | 



## list_rename

> Vec<models::RenameTrackResource> list_rename(artist_id, album_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::RenameTrackResource>**](RenameTrackResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

