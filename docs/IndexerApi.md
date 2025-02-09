# \IndexerApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_indexer**](IndexerApi.md#create_indexer) | **POST** /api/v1/indexer | 
[**create_indexer_action_by_name**](IndexerApi.md#create_indexer_action_by_name) | **POST** /api/v1/indexer/action/{name} | 
[**delete_indexer**](IndexerApi.md#delete_indexer) | **DELETE** /api/v1/indexer/{id} | 
[**delete_indexer_bulk**](IndexerApi.md#delete_indexer_bulk) | **DELETE** /api/v1/indexer/bulk | 
[**get_indexer_by_id**](IndexerApi.md#get_indexer_by_id) | **GET** /api/v1/indexer/{id} | 
[**list_indexer**](IndexerApi.md#list_indexer) | **GET** /api/v1/indexer | 
[**list_indexer_schema**](IndexerApi.md#list_indexer_schema) | **GET** /api/v1/indexer/schema | 
[**put_indexer_bulk**](IndexerApi.md#put_indexer_bulk) | **PUT** /api/v1/indexer/bulk | 
[**test_indexer**](IndexerApi.md#test_indexer) | **POST** /api/v1/indexer/test | 
[**testall_indexer**](IndexerApi.md#testall_indexer) | **POST** /api/v1/indexer/testall | 
[**update_indexer**](IndexerApi.md#update_indexer) | **PUT** /api/v1/indexer/{id} | 



## create_indexer

> models::IndexerResource create_indexer(force_save, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_indexer_action_by_name

> create_indexer_action_by_name(name, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_indexer

> delete_indexer(id)


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


## delete_indexer_bulk

> delete_indexer_bulk(indexer_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_bulk_resource** | Option<[**IndexerBulkResource**](IndexerBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_indexer_by_id

> models::IndexerResource get_indexer_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexer

> Vec<models::IndexerResource> list_indexer()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerResource>**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexer_schema

> Vec<models::IndexerResource> list_indexer_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerResource>**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_indexer_bulk

> models::IndexerResource put_indexer_bulk(indexer_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_bulk_resource** | Option<[**IndexerBulkResource**](IndexerBulkResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_indexer

> test_indexer(force_test, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_indexer

> testall_indexer()


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


## update_indexer

> models::IndexerResource update_indexer(id, force_save, indexer_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_resource** | Option<[**IndexerResource**](IndexerResource.md)> |  |  |

### Return type

[**models::IndexerResource**](IndexerResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

