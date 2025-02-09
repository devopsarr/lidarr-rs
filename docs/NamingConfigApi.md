# \NamingConfigApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_naming_config**](NamingConfigApi.md#get_naming_config) | **GET** /api/v1/config/naming | 
[**get_naming_config_by_id**](NamingConfigApi.md#get_naming_config_by_id) | **GET** /api/v1/config/naming/{id} | 
[**get_naming_config_examples**](NamingConfigApi.md#get_naming_config_examples) | **GET** /api/v1/config/naming/examples | 
[**update_naming_config**](NamingConfigApi.md#update_naming_config) | **PUT** /api/v1/config/naming/{id} | 



## get_naming_config

> models::NamingConfigResource get_naming_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_naming_config_by_id

> models::NamingConfigResource get_naming_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_naming_config_examples

> get_naming_config_examples(rename_tracks, replace_illegal_characters, colon_replacement_format, standard_track_format, multi_disc_track_format, artist_folder_format, include_artist_name, include_album_title, include_quality, replace_spaces, separator, number_style, id, resource_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rename_tracks** | Option<**bool**> |  |  |
**replace_illegal_characters** | Option<**bool**> |  |  |
**colon_replacement_format** | Option<**i32**> |  |  |
**standard_track_format** | Option<**String**> |  |  |
**multi_disc_track_format** | Option<**String**> |  |  |
**artist_folder_format** | Option<**String**> |  |  |
**include_artist_name** | Option<**bool**> |  |  |
**include_album_title** | Option<**bool**> |  |  |
**include_quality** | Option<**bool**> |  |  |
**replace_spaces** | Option<**bool**> |  |  |
**separator** | Option<**String**> |  |  |
**number_style** | Option<**String**> |  |  |
**id** | Option<**i32**> |  |  |
**resource_name** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_naming_config

> models::NamingConfigResource update_naming_config(id, naming_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**naming_config_resource** | Option<[**NamingConfigResource**](NamingConfigResource.md)> |  |  |

### Return type

[**models::NamingConfigResource**](NamingConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

