# \ImportListApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_import_list**](ImportListApi.md#create_import_list) | **POST** /api/v1/importlist | 
[**create_import_list_action_by_name**](ImportListApi.md#create_import_list_action_by_name) | **POST** /api/v1/importlist/action/{name} | 
[**delete_import_list**](ImportListApi.md#delete_import_list) | **DELETE** /api/v1/importlist/{id} | 
[**delete_import_list_bulk**](ImportListApi.md#delete_import_list_bulk) | **DELETE** /api/v1/importlist/bulk | 
[**get_import_list_by_id**](ImportListApi.md#get_import_list_by_id) | **GET** /api/v1/importlist/{id} | 
[**list_import_list**](ImportListApi.md#list_import_list) | **GET** /api/v1/importlist | 
[**list_import_list_schema**](ImportListApi.md#list_import_list_schema) | **GET** /api/v1/importlist/schema | 
[**put_import_list_bulk**](ImportListApi.md#put_import_list_bulk) | **PUT** /api/v1/importlist/bulk | 
[**test_import_list**](ImportListApi.md#test_import_list) | **POST** /api/v1/importlist/test | 
[**testall_import_list**](ImportListApi.md#testall_import_list) | **POST** /api/v1/importlist/testall | 
[**update_import_list**](ImportListApi.md#update_import_list) | **PUT** /api/v1/importlist/{id} | 



## create_import_list

> models::ImportListResource create_import_list(force_save, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_import_list_action_by_name

> create_import_list_action_by_name(name, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_import_list

> delete_import_list(id)


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


## delete_import_list_bulk

> delete_import_list_bulk(import_list_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_bulk_resource** | Option<[**ImportListBulkResource**](ImportListBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_import_list_by_id

> models::ImportListResource get_import_list_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_import_list

> Vec<models::ImportListResource> list_import_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListResource>**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_import_list_schema

> Vec<models::ImportListResource> list_import_list_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ImportListResource>**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_import_list_bulk

> models::ImportListResource put_import_list_bulk(import_list_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**import_list_bulk_resource** | Option<[**ImportListBulkResource**](ImportListBulkResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_import_list

> test_import_list(force_test, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_import_list

> testall_import_list()


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


## update_import_list

> models::ImportListResource update_import_list(id, force_save, import_list_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**import_list_resource** | Option<[**ImportListResource**](ImportListResource.md)> |  |  |

### Return type

[**models::ImportListResource**](ImportListResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

