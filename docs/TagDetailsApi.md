# \TagDetailsApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tag_detail_by_id**](TagDetailsApi.md#get_tag_detail_by_id) | **GET** /api/v1/tag/detail/{id} | 
[**list_tag_detail**](TagDetailsApi.md#list_tag_detail) | **GET** /api/v1/tag/detail | 



## get_tag_detail_by_id

> models::TagDetailsResource get_tag_detail_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::TagDetailsResource**](TagDetailsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tag_detail

> Vec<models::TagDetailsResource> list_tag_detail()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TagDetailsResource>**](TagDetailsResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

