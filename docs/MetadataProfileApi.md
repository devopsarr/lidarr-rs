# \MetadataProfileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_metadata_profile**](MetadataProfileApi.md#create_metadata_profile) | **POST** /api/v1/metadataprofile | 
[**delete_metadata_profile**](MetadataProfileApi.md#delete_metadata_profile) | **DELETE** /api/v1/metadataprofile/{id} | 
[**get_metadata_profile_by_id**](MetadataProfileApi.md#get_metadata_profile_by_id) | **GET** /api/v1/metadataprofile/{id} | 
[**list_metadata_profile**](MetadataProfileApi.md#list_metadata_profile) | **GET** /api/v1/metadataprofile | 
[**update_metadata_profile**](MetadataProfileApi.md#update_metadata_profile) | **PUT** /api/v1/metadataprofile/{id} | 



## create_metadata_profile

> models::MetadataProfileResource create_metadata_profile(metadata_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metadata_profile_resource** | Option<[**MetadataProfileResource**](MetadataProfileResource.md)> |  |  |

### Return type

[**models::MetadataProfileResource**](MetadataProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metadata_profile

> delete_metadata_profile(id)


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


## get_metadata_profile_by_id

> models::MetadataProfileResource get_metadata_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataProfileResource**](MetadataProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metadata_profile

> Vec<models::MetadataProfileResource> list_metadata_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MetadataProfileResource>**](MetadataProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata_profile

> models::MetadataProfileResource update_metadata_profile(id, metadata_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**metadata_profile_resource** | Option<[**MetadataProfileResource**](MetadataProfileResource.md)> |  |  |

### Return type

[**models::MetadataProfileResource**](MetadataProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

