# \DownloadClientApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_download_client**](DownloadClientApi.md#create_download_client) | **POST** /api/v1/downloadclient | 
[**create_download_client_action_by_name**](DownloadClientApi.md#create_download_client_action_by_name) | **POST** /api/v1/downloadclient/action/{name} | 
[**delete_download_client**](DownloadClientApi.md#delete_download_client) | **DELETE** /api/v1/downloadclient/{id} | 
[**delete_download_client_bulk**](DownloadClientApi.md#delete_download_client_bulk) | **DELETE** /api/v1/downloadclient/bulk | 
[**get_download_client_by_id**](DownloadClientApi.md#get_download_client_by_id) | **GET** /api/v1/downloadclient/{id} | 
[**list_download_client**](DownloadClientApi.md#list_download_client) | **GET** /api/v1/downloadclient | 
[**list_download_client_schema**](DownloadClientApi.md#list_download_client_schema) | **GET** /api/v1/downloadclient/schema | 
[**put_download_client_bulk**](DownloadClientApi.md#put_download_client_bulk) | **PUT** /api/v1/downloadclient/bulk | 
[**test_download_client**](DownloadClientApi.md#test_download_client) | **POST** /api/v1/downloadclient/test | 
[**testall_download_client**](DownloadClientApi.md#testall_download_client) | **POST** /api/v1/downloadclient/testall | 
[**update_download_client**](DownloadClientApi.md#update_download_client) | **PUT** /api/v1/downloadclient/{id} | 



## create_download_client

> models::DownloadClientResource create_download_client(force_save, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_download_client_action_by_name

> create_download_client_action_by_name(name, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_download_client

> delete_download_client(id)


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


## delete_download_client_bulk

> delete_download_client_bulk(download_client_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_client_bulk_resource** | Option<[**DownloadClientBulkResource**](DownloadClientBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_download_client_by_id

> models::DownloadClientResource get_download_client_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_download_client

> Vec<models::DownloadClientResource> list_download_client()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DownloadClientResource>**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_download_client_schema

> Vec<models::DownloadClientResource> list_download_client_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DownloadClientResource>**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_download_client_bulk

> models::DownloadClientResource put_download_client_bulk(download_client_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_client_bulk_resource** | Option<[**DownloadClientBulkResource**](DownloadClientBulkResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_download_client

> test_download_client(force_test, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_download_client

> testall_download_client()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_download_client

> models::DownloadClientResource update_download_client(id, force_save, download_client_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**download_client_resource** | Option<[**DownloadClientResource**](DownloadClientResource.md)> |  |  |

### Return type

[**models::DownloadClientResource**](DownloadClientResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

