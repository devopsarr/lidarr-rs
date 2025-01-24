# \CalendarApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_calendar_by_id**](CalendarApi.md#get_calendar_by_id) | **GET** /api/v1/calendar/{id} | 
[**list_calendar**](CalendarApi.md#list_calendar) | **GET** /api/v1/calendar | 



## get_calendar_by_id

> models::AlbumResource get_calendar_by_id(id)


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


## list_calendar

> Vec<models::AlbumResource> list_calendar(start, end, unmonitored, include_artist, tags)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | Option<**String**> |  |  |
**end** | Option<**String**> |  |  |
**unmonitored** | Option<**bool**> |  |  |[default to false]
**include_artist** | Option<**bool**> |  |  |[default to false]
**tags** | Option<**String**> |  |  |[default to ]

### Return type

[**Vec<models::AlbumResource>**](AlbumResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

