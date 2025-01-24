# \MediaManagementConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_media_management_config**](MediaManagementConfigApi.md#get_media_management_config) | **GET** /api/v1/config/mediamanagement | 
[**get_media_management_config_by_id**](MediaManagementConfigApi.md#get_media_management_config_by_id) | **GET** /api/v1/config/mediamanagement/{id} | 
[**update_media_management_config**](MediaManagementConfigApi.md#update_media_management_config) | **PUT** /api/v1/config/mediamanagement/{id} | 



## get_media_management_config

> models::MediaManagementConfigResource get_media_management_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_management_config_by_id

> models::MediaManagementConfigResource get_media_management_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_media_management_config

> models::MediaManagementConfigResource update_media_management_config(id, media_management_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**media_management_config_resource** | Option<[**MediaManagementConfigResource**](MediaManagementConfigResource.md)> |  |  |

### Return type

[**models::MediaManagementConfigResource**](MediaManagementConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

