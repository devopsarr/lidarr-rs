# \ReleaseApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_release**](ReleaseApi.md#create_release) | **POST** /api/v1/release | 
[**list_release**](ReleaseApi.md#list_release) | **GET** /api/v1/release | 



## create_release

> models::ReleaseResource create_release(release_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**release_resource** | Option<[**ReleaseResource**](ReleaseResource.md)> |  |  |

### Return type

[**models::ReleaseResource**](ReleaseResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_release

> Vec<models::ReleaseResource> list_release(album_id, artist_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_id** | Option<**i32**> |  |  |
**artist_id** | Option<**i32**> |  |  |

### Return type

[**Vec<models::ReleaseResource>**](ReleaseResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

