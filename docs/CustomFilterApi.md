# \CustomFilterApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_filter**](CustomFilterApi.md#create_custom_filter) | **POST** /api/v1/customfilter | 
[**delete_custom_filter**](CustomFilterApi.md#delete_custom_filter) | **DELETE** /api/v1/customfilter/{id} | 
[**get_custom_filter_by_id**](CustomFilterApi.md#get_custom_filter_by_id) | **GET** /api/v1/customfilter/{id} | 
[**list_custom_filter**](CustomFilterApi.md#list_custom_filter) | **GET** /api/v1/customfilter | 
[**update_custom_filter**](CustomFilterApi.md#update_custom_filter) | **PUT** /api/v1/customfilter/{id} | 



## create_custom_filter

> models::CustomFilterResource create_custom_filter(custom_filter_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_filter_resource** | Option<[**CustomFilterResource**](CustomFilterResource.md)> |  |  |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_filter

> delete_custom_filter(id)


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


## get_custom_filter_by_id

> models::CustomFilterResource get_custom_filter_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_custom_filter

> Vec<models::CustomFilterResource> list_custom_filter()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CustomFilterResource>**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_filter

> models::CustomFilterResource update_custom_filter(id, custom_filter_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**custom_filter_resource** | Option<[**CustomFilterResource**](CustomFilterResource.md)> |  |  |

### Return type

[**models::CustomFilterResource**](CustomFilterResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

