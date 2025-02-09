# \ReleaseProfileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_release_profile**](ReleaseProfileApi.md#create_release_profile) | **POST** /api/v1/releaseprofile | 
[**delete_release_profile**](ReleaseProfileApi.md#delete_release_profile) | **DELETE** /api/v1/releaseprofile/{id} | 
[**get_release_profile_by_id**](ReleaseProfileApi.md#get_release_profile_by_id) | **GET** /api/v1/releaseprofile/{id} | 
[**list_release_profile**](ReleaseProfileApi.md#list_release_profile) | **GET** /api/v1/releaseprofile | 
[**update_release_profile**](ReleaseProfileApi.md#update_release_profile) | **PUT** /api/v1/releaseprofile/{id} | 



## create_release_profile

> models::ReleaseProfileResource create_release_profile(release_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**release_profile_resource** | Option<[**ReleaseProfileResource**](ReleaseProfileResource.md)> |  |  |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_release_profile

> delete_release_profile(id)


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


## get_release_profile_by_id

> models::ReleaseProfileResource get_release_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_release_profile

> Vec<models::ReleaseProfileResource> list_release_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ReleaseProfileResource>**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_release_profile

> models::ReleaseProfileResource update_release_profile(id, release_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**release_profile_resource** | Option<[**ReleaseProfileResource**](ReleaseProfileResource.md)> |  |  |

### Return type

[**models::ReleaseProfileResource**](ReleaseProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

