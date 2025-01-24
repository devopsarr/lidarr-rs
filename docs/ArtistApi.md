# \ArtistApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_artist**](ArtistApi.md#create_artist) | **POST** /api/v1/artist | 
[**delete_artist**](ArtistApi.md#delete_artist) | **DELETE** /api/v1/artist/{id} | 
[**get_artist_by_id**](ArtistApi.md#get_artist_by_id) | **GET** /api/v1/artist/{id} | 
[**list_artist**](ArtistApi.md#list_artist) | **GET** /api/v1/artist | 
[**update_artist**](ArtistApi.md#update_artist) | **PUT** /api/v1/artist/{id} | 



## create_artist

> models::ArtistResource create_artist(artist_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_resource** | Option<[**ArtistResource**](ArtistResource.md)> |  |  |

### Return type

[**models::ArtistResource**](ArtistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_artist

> delete_artist(id, delete_files, add_import_list_exclusion)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**delete_files** | Option<**bool**> |  |  |[default to false]
**add_import_list_exclusion** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_artist_by_id

> models::ArtistResource get_artist_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ArtistResource**](ArtistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_artist

> Vec<models::ArtistResource> list_artist(mb_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mb_id** | Option<**uuid::Uuid**> |  |  |

### Return type

[**Vec<models::ArtistResource>**](ArtistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_artist

> models::ArtistResource update_artist(id, move_files, artist_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**move_files** | Option<**bool**> |  |  |[default to false]
**artist_resource** | Option<[**ArtistResource**](ArtistResource.md)> |  |  |

### Return type

[**models::ArtistResource**](ArtistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

