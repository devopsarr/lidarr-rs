# \IndexerConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_indexer_config**](IndexerConfigApi.md#get_indexer_config) | **GET** /api/v1/config/indexer | 
[**get_indexer_config_by_id**](IndexerConfigApi.md#get_indexer_config_by_id) | **GET** /api/v1/config/indexer/{id} | 
[**update_indexer_config**](IndexerConfigApi.md#update_indexer_config) | **PUT** /api/v1/config/indexer/{id} | 



## get_indexer_config

> models::IndexerConfigResource get_indexer_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_indexer_config_by_id

> models::IndexerConfigResource get_indexer_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_indexer_config

> models::IndexerConfigResource update_indexer_config(id, indexer_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**indexer_config_resource** | Option<[**IndexerConfigResource**](IndexerConfigResource.md)> |  |  |

### Return type

[**models::IndexerConfigResource**](IndexerConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

