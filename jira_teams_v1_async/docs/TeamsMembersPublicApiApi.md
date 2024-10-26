# \TeamsMembersPublicApiApi

All URIs are relative to *https://soupcop.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_members2**](TeamsMembersPublicApiApi.md#add_members2) | **POST** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId}/members/add | Add a set of membership(s).
[**fetch_members**](TeamsMembersPublicApiApi.md#fetch_members) | **POST** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId}/members | Fetch a set of membership(s).
[**remove_members1**](TeamsMembersPublicApiApi.md#remove_members1) | **POST** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId}/members/remove | Remove a set of membership(s).



## add_members2

> models::PublicApiMembershipAddResponse add_members2(org_id, team_id, public_api_membership_add_payload)
Add a set of membership(s).

The account IDs specified will be added to the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation of the team you are adding members to. | [required] |
**team_id** | **String** | The ID of the team you are adding members to. | [required] |
**public_api_membership_add_payload** | [**PublicApiMembershipAddPayload**](PublicApiMembershipAddPayload.md) | A set of account IDs to add as members. | [required] |

### Return type

[**models::PublicApiMembershipAddResponse**](PublicApiMembershipAddResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_members

> models::PublicApiFetchResponsePublicApiMembershipAccountId fetch_members(org_id, team_id, site_id, public_api_membership_fetch_payload)
Fetch a set of membership(s).

Returns a set of account IDs who are members of the team, alongside a pagination cursor to retrieve the next page (if available).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation of the team you are fetching members for. | [required] |
**team_id** | **String** | The ID of the team you are fetching members for. | [required] |
**site_id** | Option<**String**> | [Optional] The ID of the site you are fetching members for. |  |
**public_api_membership_fetch_payload** | Option<[**PublicApiMembershipFetchPayload**](PublicApiMembershipFetchPayload.md)> | Optional Relay-style pagination controls. Can be omitted if empty. |  |

### Return type

[**models::PublicApiFetchResponsePublicApiMembershipAccountId**](PublicApiFetchResponsePublicApiMembershipAccountId.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_members1

> models::PublicApiMembershipRemoveResponse remove_members1(org_id, team_id, public_api_membership_remove_payload)
Remove a set of membership(s).

The account IDs specified will be removed from the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation of the team you are removing members from. | [required] |
**team_id** | **String** | The ID of the team you are removing members from. | [required] |
**public_api_membership_remove_payload** | [**PublicApiMembershipRemovePayload**](PublicApiMembershipRemovePayload.md) | A set of account IDs to remove as members. | [required] |

### Return type

[**models::PublicApiMembershipRemoveResponse**](PublicApiMembershipRemoveResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

