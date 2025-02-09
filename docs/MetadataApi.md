# \MetadataApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_metadata**](MetadataApi.md#create_metadata) | **POST** /api/v1/metadata | 
[**create_metadata_action_by_name**](MetadataApi.md#create_metadata_action_by_name) | **POST** /api/v1/metadata/action/{name} | 
[**delete_metadata**](MetadataApi.md#delete_metadata) | **DELETE** /api/v1/metadata/{id} | 
[**get_metadata_by_id**](MetadataApi.md#get_metadata_by_id) | **GET** /api/v1/metadata/{id} | 
[**list_metadata**](MetadataApi.md#list_metadata) | **GET** /api/v1/metadata | 
[**list_metadata_schema**](MetadataApi.md#list_metadata_schema) | **GET** /api/v1/metadata/schema | 
[**test_metadata**](MetadataApi.md#test_metadata) | **POST** /api/v1/metadata/test | 
[**testall_metadata**](MetadataApi.md#testall_metadata) | **POST** /api/v1/metadata/testall | 
[**update_metadata**](MetadataApi.md#update_metadata) | **PUT** /api/v1/metadata/{id} | 



## create_metadata

> models::MetadataResource create_metadata(force_save, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_metadata_action_by_name

> create_metadata_action_by_name(name, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_metadata

> delete_metadata(id)


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


## get_metadata_by_id

> models::MetadataResource get_metadata_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metadata

> Vec<models::MetadataResource> list_metadata()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MetadataResource>**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_metadata_schema

> Vec<models::MetadataResource> list_metadata_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::MetadataResource>**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_metadata

> test_metadata(force_test, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_metadata

> testall_metadata()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata

> models::MetadataResource update_metadata(id, force_save, metadata_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**metadata_resource** | Option<[**MetadataResource**](MetadataResource.md)> |  |  |

### Return type

[**models::MetadataResource**](MetadataResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

