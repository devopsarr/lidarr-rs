# \DownloadClientConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_download_client_config**](DownloadClientConfigApi.md#get_download_client_config) | **GET** /api/v1/config/downloadclient | 
[**get_download_client_config_by_id**](DownloadClientConfigApi.md#get_download_client_config_by_id) | **GET** /api/v1/config/downloadclient/{id} | 
[**update_download_client_config**](DownloadClientConfigApi.md#update_download_client_config) | **PUT** /api/v1/config/downloadclient/{id} | 



## get_download_client_config

> models::DownloadClientConfigResource get_download_client_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_client_config_by_id

> models::DownloadClientConfigResource get_download_client_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_download_client_config

> models::DownloadClientConfigResource update_download_client_config(id, download_client_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**download_client_config_resource** | Option<[**DownloadClientConfigResource**](DownloadClientConfigResource.md)> |  |  |

### Return type

[**models::DownloadClientConfigResource**](DownloadClientConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

