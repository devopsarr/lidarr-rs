# \QualityProfileApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_quality_profile**](QualityProfileApi.md#create_quality_profile) | **POST** /api/v1/qualityprofile | 
[**delete_quality_profile**](QualityProfileApi.md#delete_quality_profile) | **DELETE** /api/v1/qualityprofile/{id} | 
[**get_quality_profile_by_id**](QualityProfileApi.md#get_quality_profile_by_id) | **GET** /api/v1/qualityprofile/{id} | 
[**list_quality_profile**](QualityProfileApi.md#list_quality_profile) | **GET** /api/v1/qualityprofile | 
[**update_quality_profile**](QualityProfileApi.md#update_quality_profile) | **PUT** /api/v1/qualityprofile/{id} | 



## create_quality_profile

> models::QualityProfileResource create_quality_profile(quality_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quality_profile_resource** | Option<[**QualityProfileResource**](QualityProfileResource.md)> |  |  |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quality_profile

> delete_quality_profile(id)


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


## get_quality_profile_by_id

> models::QualityProfileResource get_quality_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_quality_profile

> Vec<models::QualityProfileResource> list_quality_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::QualityProfileResource>**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_quality_profile

> models::QualityProfileResource update_quality_profile(id, quality_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**quality_profile_resource** | Option<[**QualityProfileResource**](QualityProfileResource.md)> |  |  |

### Return type

[**models::QualityProfileResource**](QualityProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

