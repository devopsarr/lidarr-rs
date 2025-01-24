# \RootFolderApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_root_folder**](RootFolderApi.md#create_root_folder) | **POST** /api/v1/rootfolder | 
[**delete_root_folder**](RootFolderApi.md#delete_root_folder) | **DELETE** /api/v1/rootfolder/{id} | 
[**get_root_folder_by_id**](RootFolderApi.md#get_root_folder_by_id) | **GET** /api/v1/rootfolder/{id} | 
[**list_root_folder**](RootFolderApi.md#list_root_folder) | **GET** /api/v1/rootfolder | 
[**update_root_folder**](RootFolderApi.md#update_root_folder) | **PUT** /api/v1/rootfolder/{id} | 



## create_root_folder

> models::RootFolderResource create_root_folder(root_folder_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_folder_resource** | Option<[**RootFolderResource**](RootFolderResource.md)> |  |  |

### Return type

[**models::RootFolderResource**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_root_folder

> delete_root_folder(id)


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


## get_root_folder_by_id

> models::RootFolderResource get_root_folder_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::RootFolderResource**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_root_folder

> Vec<models::RootFolderResource> list_root_folder()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::RootFolderResource>**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_root_folder

> models::RootFolderResource update_root_folder(id, root_folder_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**root_folder_resource** | Option<[**RootFolderResource**](RootFolderResource.md)> |  |  |

### Return type

[**models::RootFolderResource**](RootFolderResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

