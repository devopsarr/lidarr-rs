# \AlbumApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_album**](AlbumApi.md#create_album) | **POST** /api/v1/album | 
[**delete_album**](AlbumApi.md#delete_album) | **DELETE** /api/v1/album/{id} | 
[**get_album_by_id**](AlbumApi.md#get_album_by_id) | **GET** /api/v1/album/{id} | 
[**list_album**](AlbumApi.md#list_album) | **GET** /api/v1/album | 
[**put_album_monitor**](AlbumApi.md#put_album_monitor) | **PUT** /api/v1/album/monitor | 
[**update_album**](AlbumApi.md#update_album) | **PUT** /api/v1/album/{id} | 



## create_album

> models::AlbumResource create_album(album_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**album_resource** | Option<[**AlbumResource**](AlbumResource.md)> |  |  |

### Return type

[**models::AlbumResource**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_album

> delete_album(id, delete_files, add_import_list_exclusion)


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


## get_album_by_id

> models::AlbumResource get_album_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AlbumResource**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_album

> Vec<models::AlbumResource> list_album(artist_id, album_ids, foreign_album_id, include_all_artist_albums)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**artist_id** | Option<**i32**> |  |  |
**album_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |
**foreign_album_id** | Option<**String**> |  |  |
**include_all_artist_albums** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<models::AlbumResource>**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_album_monitor

> put_album_monitor(albums_monitored_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**albums_monitored_resource** | Option<[**AlbumsMonitoredResource**](AlbumsMonitoredResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_album

> models::AlbumResource update_album(id, album_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**album_resource** | Option<[**AlbumResource**](AlbumResource.md)> |  |  |

### Return type

[**models::AlbumResource**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

