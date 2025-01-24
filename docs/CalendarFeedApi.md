# \CalendarFeedApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_feed_v1_calendar_lidarr_period_ics**](CalendarFeedApi.md#get_feed_v1_calendar_lidarr_period_ics) | **GET** /feed/v1/calendar/lidarr.ics | 



## get_feed_v1_calendar_lidarr_period_ics

> get_feed_v1_calendar_lidarr_period_ics(past_days, future_days, tags, unmonitored)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**past_days** | Option<**i32**> |  |  |[default to 7]
**future_days** | Option<**i32**> |  |  |[default to 28]
**tags** | Option<**String**> |  |  |[default to ]
**unmonitored** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

