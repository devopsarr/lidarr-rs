# \QualityDefinitionApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_quality_definition_by_id**](QualityDefinitionApi.md#get_quality_definition_by_id) | **GET** /api/v1/qualitydefinition/{id} | 
[**list_quality_definition**](QualityDefinitionApi.md#list_quality_definition) | **GET** /api/v1/qualitydefinition | 
[**put_quality_definition_update**](QualityDefinitionApi.md#put_quality_definition_update) | **PUT** /api/v1/qualitydefinition/update | 
[**update_quality_definition**](QualityDefinitionApi.md#update_quality_definition) | **PUT** /api/v1/qualitydefinition/{id} | 



## get_quality_definition_by_id

> models::QualityDefinitionResource get_quality_definition_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::QualityDefinitionResource**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_quality_definition

> Vec<models::QualityDefinitionResource> list_quality_definition()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QualityDefinitionResource>**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_definition_update

> put_quality_definition_update(quality_definition_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quality_definition_resource** | Option<[**Vec<models::QualityDefinitionResource>**](QualityDefinitionResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_quality_definition

> models::QualityDefinitionResource update_quality_definition(id, quality_definition_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**quality_definition_resource** | Option<[**QualityDefinitionResource**](QualityDefinitionResource.md)> |  |  |

### Return type

[**models::QualityDefinitionResource**](QualityDefinitionResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

