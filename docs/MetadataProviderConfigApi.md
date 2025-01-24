# \MetadataProviderConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_metadata_provider_config**](MetadataProviderConfigApi.md#get_metadata_provider_config) | **GET** /api/v1/config/metadataprovider | 
[**get_metadata_provider_config_by_id**](MetadataProviderConfigApi.md#get_metadata_provider_config_by_id) | **GET** /api/v1/config/metadataprovider/{id} | 
[**update_metadata_provider_config**](MetadataProviderConfigApi.md#update_metadata_provider_config) | **PUT** /api/v1/config/metadataprovider/{id} | 



## get_metadata_provider_config

> models::MetadataProviderConfigResource get_metadata_provider_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MetadataProviderConfigResource**](MetadataProviderConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_metadata_provider_config_by_id

> models::MetadataProviderConfigResource get_metadata_provider_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::MetadataProviderConfigResource**](MetadataProviderConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_metadata_provider_config

> models::MetadataProviderConfigResource update_metadata_provider_config(id, metadata_provider_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**metadata_provider_config_resource** | Option<[**MetadataProviderConfigResource**](MetadataProviderConfigResource.md)> |  |  |

### Return type

[**models::MetadataProviderConfigResource**](MetadataProviderConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

