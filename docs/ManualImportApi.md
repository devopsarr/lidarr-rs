# \ManualImportApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_manual_import**](ManualImportApi.md#create_manual_import) | **POST** /api/v1/manualimport | 
[**list_manual_import**](ManualImportApi.md#list_manual_import) | **GET** /api/v1/manualimport | 



## create_manual_import

> create_manual_import(manual_import_update_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**manual_import_update_resource** | Option<[**Vec<models::ManualImportUpdateResource>**](ManualImportUpdateResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_manual_import

> Vec<models::ManualImportResource> list_manual_import(folder, download_id, artist_id, filter_existing_files, replace_existing_files)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder** | Option<**String**> |  |  |
**download_id** | Option<**String**> |  |  |
**artist_id** | Option<**i32**> |  |  |
**filter_existing_files** | Option<**bool**> |  |  |[default to true]
**replace_existing_files** | Option<**bool**> |  |  |[default to true]

### Return type

[**Vec<models::ManualImportResource>**](ManualImportResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

