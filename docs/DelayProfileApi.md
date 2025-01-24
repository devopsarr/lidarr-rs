# \DelayProfileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_delay_profile**](DelayProfileApi.md#create_delay_profile) | **POST** /api/v1/delayprofile | 
[**delete_delay_profile**](DelayProfileApi.md#delete_delay_profile) | **DELETE** /api/v1/delayprofile/{id} | 
[**get_delay_profile_by_id**](DelayProfileApi.md#get_delay_profile_by_id) | **GET** /api/v1/delayprofile/{id} | 
[**list_delay_profile**](DelayProfileApi.md#list_delay_profile) | **GET** /api/v1/delayprofile | 
[**update_delay_profile**](DelayProfileApi.md#update_delay_profile) | **PUT** /api/v1/delayprofile/{id} | 
[**update_delay_profile_reorder**](DelayProfileApi.md#update_delay_profile_reorder) | **PUT** /api/v1/delayprofile/reorder/{id} | 



## create_delay_profile

> models::DelayProfileResource create_delay_profile(delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_delay_profile

> delete_delay_profile(id)


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


## get_delay_profile_by_id

> models::DelayProfileResource get_delay_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_delay_profile

> Vec<models::DelayProfileResource> list_delay_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DelayProfileResource>**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delay_profile

> models::DelayProfileResource update_delay_profile(id, delay_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delay_profile_resource** | Option<[**DelayProfileResource**](DelayProfileResource.md)> |  |  |

### Return type

[**models::DelayProfileResource**](DelayProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_delay_profile_reorder

> update_delay_profile_reorder(id, after_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**after_id** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

