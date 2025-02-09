# \UpdateLogFileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_log_file_update_by_filename**](UpdateLogFileApi.md#get_log_file_update_by_filename) | **GET** /api/v1/log/file/update/{filename} | 
[**list_log_file_update**](UpdateLogFileApi.md#list_log_file_update) | **GET** /api/v1/log/file/update | 



## get_log_file_update_by_filename

> get_log_file_update_by_filename(filename)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filename** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_log_file_update

> Vec<models::LogFileResource> list_log_file_update()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LogFileResource>**](LogFileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

