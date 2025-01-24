# \RemotePathMappingApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_remote_path_mapping**](RemotePathMappingApi.md#create_remote_path_mapping) | **POST** /api/v1/remotepathmapping | 
[**delete_remote_path_mapping**](RemotePathMappingApi.md#delete_remote_path_mapping) | **DELETE** /api/v1/remotepathmapping/{id} | 
[**get_remote_path_mapping_by_id**](RemotePathMappingApi.md#get_remote_path_mapping_by_id) | **GET** /api/v1/remotepathmapping/{id} | 
[**list_remote_path_mapping**](RemotePathMappingApi.md#list_remote_path_mapping) | **GET** /api/v1/remotepathmapping | 
[**update_remote_path_mapping**](RemotePathMappingApi.md#update_remote_path_mapping) | **PUT** /api/v1/remotepathmapping/{id} | 



## create_remote_path_mapping

> models::RemotePathMappingResource create_remote_path_mapping(remote_path_mapping_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**remote_path_mapping_resource** | Option<[**RemotePathMappingResource**](RemotePathMappingResource.md)> |  |  |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_remote_path_mapping

> delete_remote_path_mapping(id)


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


## get_remote_path_mapping_by_id

> models::RemotePathMappingResource get_remote_path_mapping_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_remote_path_mapping

> Vec<models::RemotePathMappingResource> list_remote_path_mapping()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RemotePathMappingResource>**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_remote_path_mapping

> models::RemotePathMappingResource update_remote_path_mapping(id, remote_path_mapping_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**remote_path_mapping_resource** | Option<[**RemotePathMappingResource**](RemotePathMappingResource.md)> |  |  |

### Return type

[**models::RemotePathMappingResource**](RemotePathMappingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

