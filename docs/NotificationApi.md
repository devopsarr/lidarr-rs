# \NotificationApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_notification**](NotificationApi.md#create_notification) | **POST** /api/v1/notification | 
[**create_notification_action_by_name**](NotificationApi.md#create_notification_action_by_name) | **POST** /api/v1/notification/action/{name} | 
[**delete_notification**](NotificationApi.md#delete_notification) | **DELETE** /api/v1/notification/{id} | 
[**get_notification_by_id**](NotificationApi.md#get_notification_by_id) | **GET** /api/v1/notification/{id} | 
[**list_notification**](NotificationApi.md#list_notification) | **GET** /api/v1/notification | 
[**list_notification_schema**](NotificationApi.md#list_notification_schema) | **GET** /api/v1/notification/schema | 
[**test_notification**](NotificationApi.md#test_notification) | **POST** /api/v1/notification/test | 
[**testall_notification**](NotificationApi.md#testall_notification) | **POST** /api/v1/notification/testall | 
[**update_notification**](NotificationApi.md#update_notification) | **PUT** /api/v1/notification/{id} | 



## create_notification

> models::NotificationResource create_notification(force_save, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_notification_action_by_name

> create_notification_action_by_name(name, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification

> delete_notification(id)


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


## get_notification_by_id

> models::NotificationResource get_notification_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notification

> Vec<models::NotificationResource> list_notification()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationResource>**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notification_schema

> Vec<models::NotificationResource> list_notification_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::NotificationResource>**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_notification

> test_notification(force_test, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_notification

> testall_notification()


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


## update_notification

> models::NotificationResource update_notification(id, force_save, notification_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**notification_resource** | Option<[**NotificationResource**](NotificationResource.md)> |  |  |

### Return type

[**models::NotificationResource**](NotificationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

