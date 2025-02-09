# \HostConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_host_config**](HostConfigApi.md#get_host_config) | **GET** /api/v1/config/host | 
[**get_host_config_by_id**](HostConfigApi.md#get_host_config_by_id) | **GET** /api/v1/config/host/{id} | 
[**update_host_config**](HostConfigApi.md#update_host_config) | **PUT** /api/v1/config/host/{id} | 



## get_host_config

> models::HostConfigResource get_host_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_host_config_by_id

> models::HostConfigResource get_host_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_host_config

> models::HostConfigResource update_host_config(id, host_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**host_config_resource** | Option<[**HostConfigResource**](HostConfigResource.md)> |  |  |

### Return type

[**models::HostConfigResource**](HostConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

