# \ArtistLookupApi

All URIs are relative to *http://localhost:8686*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_artist_lookup**](ArtistLookupApi.md#list_artist_lookup) | **GET** /api/v1/artist/lookup | 



## list_artist_lookup

> Vec<models::ArtistResource> list_artist_lookup(term)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | Option<**String**> |  |  |

### Return type

[**Vec<models::ArtistResource>**](ArtistResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

