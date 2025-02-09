# \AutoTaggingApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_auto_tagging**](AutoTaggingApi.md#create_auto_tagging) | **POST** /api/v1/autotagging | 
[**delete_auto_tagging**](AutoTaggingApi.md#delete_auto_tagging) | **DELETE** /api/v1/autotagging/{id} | 
[**get_auto_tagging_by_id**](AutoTaggingApi.md#get_auto_tagging_by_id) | **GET** /api/v1/autotagging/{id} | 
[**list_auto_tagging**](AutoTaggingApi.md#list_auto_tagging) | **GET** /api/v1/autotagging | 
[**list_auto_tagging_schema**](AutoTaggingApi.md#list_auto_tagging_schema) | **GET** /api/v1/autotagging/schema | 
[**update_auto_tagging**](AutoTaggingApi.md#update_auto_tagging) | **PUT** /api/v1/autotagging/{id} | 



## create_auto_tagging

> models::AutoTaggingResource create_auto_tagging(auto_tagging_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auto_tagging_resource** | Option<[**AutoTaggingResource**](AutoTaggingResource.md)> |  |  |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_auto_tagging

> delete_auto_tagging(id)


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


## get_auto_tagging_by_id

> models::AutoTaggingResource get_auto_tagging_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auto_tagging

> Vec<models::AutoTaggingResource> list_auto_tagging()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AutoTaggingResource>**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_auto_tagging_schema

> Vec<models::AutoTaggingSpecificationSchema> list_auto_tagging_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AutoTaggingSpecificationSchema>**](AutoTaggingSpecificationSchema.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_auto_tagging

> models::AutoTaggingResource update_auto_tagging(id, auto_tagging_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**auto_tagging_resource** | Option<[**AutoTaggingResource**](AutoTaggingResource.md)> |  |  |

### Return type

[**models::AutoTaggingResource**](AutoTaggingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

