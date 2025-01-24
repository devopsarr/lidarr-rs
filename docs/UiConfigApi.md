# \UiConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_ui_config**](UiConfigApi.md#get_ui_config) | **GET** /api/v1/config/ui | 
[**get_ui_config_by_id**](UiConfigApi.md#get_ui_config_by_id) | **GET** /api/v1/config/ui/{id} | 
[**update_ui_config**](UiConfigApi.md#update_ui_config) | **PUT** /api/v1/config/ui/{id} | 



## get_ui_config

> models::UiConfigResource get_ui_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ui_config_by_id

> models::UiConfigResource get_ui_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ui_config

> models::UiConfigResource update_ui_config(id, ui_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**ui_config_resource** | Option<[**UiConfigResource**](UiConfigResource.md)> |  |  |

### Return type

[**models::UiConfigResource**](UiConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

