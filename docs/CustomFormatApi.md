# \CustomFormatApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_format**](CustomFormatApi.md#create_custom_format) | **POST** /api/v1/customformat | 
[**delete_custom_format**](CustomFormatApi.md#delete_custom_format) | **DELETE** /api/v1/customformat/{id} | 
[**delete_custom_format_bulk**](CustomFormatApi.md#delete_custom_format_bulk) | **DELETE** /api/v1/customformat/bulk | 
[**get_custom_format_by_id**](CustomFormatApi.md#get_custom_format_by_id) | **GET** /api/v1/customformat/{id} | 
[**list_custom_format**](CustomFormatApi.md#list_custom_format) | **GET** /api/v1/customformat | 
[**list_custom_format_schema**](CustomFormatApi.md#list_custom_format_schema) | **GET** /api/v1/customformat/schema | 
[**put_custom_format_bulk**](CustomFormatApi.md#put_custom_format_bulk) | **PUT** /api/v1/customformat/bulk | 
[**update_custom_format**](CustomFormatApi.md#update_custom_format) | **PUT** /api/v1/customformat/{id} | 



## create_custom_format

> models::CustomFormatResource create_custom_format(custom_format_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_resource** | Option<[**CustomFormatResource**](CustomFormatResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_format

> delete_custom_format(id)


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


## delete_custom_format_bulk

> delete_custom_format_bulk(custom_format_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_bulk_resource** | Option<[**CustomFormatBulkResource**](CustomFormatBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_format_by_id

> models::CustomFormatResource get_custom_format_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_custom_format

> Vec<models::CustomFormatResource> list_custom_format()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomFormatResource>**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_custom_format_schema

> Vec<models::CustomFormatSpecificationSchema> list_custom_format_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomFormatSpecificationSchema>**](CustomFormatSpecificationSchema.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_custom_format_bulk

> models::CustomFormatResource put_custom_format_bulk(custom_format_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_format_bulk_resource** | Option<[**CustomFormatBulkResource**](CustomFormatBulkResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_format

> models::CustomFormatResource update_custom_format(id, custom_format_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**custom_format_resource** | Option<[**CustomFormatResource**](CustomFormatResource.md)> |  |  |

### Return type

[**models::CustomFormatResource**](CustomFormatResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

