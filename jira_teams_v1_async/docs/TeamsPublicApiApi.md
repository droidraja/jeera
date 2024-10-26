# \TeamsPublicApiApi

All URIs are relative to *https://soupcop.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_team**](TeamsPublicApiApi.md#create_team) | **POST** /gateway/api/public/teams/v1/org/{orgId}/teams/ | Create a team.
[**delete_team2**](TeamsPublicApiApi.md#delete_team2) | **DELETE** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId} | Delete a team.
[**get_team2**](TeamsPublicApiApi.md#get_team2) | **GET** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId} | Get a single team.
[**query_teams**](TeamsPublicApiApi.md#query_teams) | **GET** /gateway/api/public/teams/v1/org/{orgId}/teams | Get a list of teams.
[**restore_team**](TeamsPublicApiApi.md#restore_team) | **POST** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId}/restore | Restore a single soft-deleted team
[**update_team1**](TeamsPublicApiApi.md#update_team1) | **PATCH** /gateway/api/public/teams/v1/org/{orgId}/teams/{teamId} | Modify a team.
[**upload_and_set_team_cover_photo**](TeamsPublicApiApi.md#upload_and_set_team_cover_photo) | **PUT** /gateway/api/public/teams/v1/{teamId}/cover-photo | Upload a team cover photo



## create_team

> models::PublicApiTeamResponseWithMembers create_team(org_id, public_api_team_creation_payload)
Create a team.

Creates a team, and adds the requesting user as the initial member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the team is to be created under. | [required] |
**public_api_team_creation_payload** | [**PublicApiTeamCreationPayload**](PublicApiTeamCreationPayload.md) | Details of the team to be created. | [required] |

### Return type

[**models::PublicApiTeamResponseWithMembers**](PublicApiTeamResponseWithMembers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_team2

> delete_team2(org_id, team_id)
Delete a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the team is to be deleted from. | [required] |
**team_id** | **String** | The ID of the team to be deleted. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_team2

> models::PublicApiTeamResponse get_team2(org_id, team_id, site_id)
Get a single team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the team is to be retrieved from. | [required] |
**team_id** | **String** | The ID of the team to be retrieved. | [required] |
**site_id** | Option<**String**> | [Optional] The ID of the site to retrieve teams which are site scoped. |  |

### Return type

[**models::PublicApiTeamResponse**](PublicApiTeamResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_teams

> models::PublicApiTeamPaginationResult query_teams(org_id, site_id, size, cursor)
Get a list of teams.

This returns a list of all teams contained under an organization. This may be used as an option to export teams data within your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the teams are to be retrieved from. | [required] |
**site_id** | Option<**String**> | [Optional] The ID of the site to retrieve teams which are site scoped. |  |
**size** | Option<**u32**> | The page size for the number of teams to return (max 300) |  |[default to 50]
**cursor** | Option<**String**> | An optional cursor token. Leave off for the first request. |  |

### Return type

[**models::PublicApiTeamPaginationResult**](PublicApiTeamPaginationResult.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_team

> restore_team(org_id, team_id)
Restore a single soft-deleted team

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the team belongs to | [required] |
**team_id** | **String** | The ID of the team to restore | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_team1

> models::PublicApiTeamResponse update_team1(org_id, team_id, public_api_team_update_payload)
Modify a team.

This will only update the fields that get passed in and leave the rest as unmodified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **String** | The ID of the organisation the team to be updated belongs to. | [required] |
**team_id** | **String** | The ID of the team to be updated. | [required] |
**public_api_team_update_payload** | [**PublicApiTeamUpdatePayload**](PublicApiTeamUpdatePayload.md) | Details the team is to be updated with. | [required] |

### Return type

[**models::PublicApiTeamResponse**](PublicApiTeamResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_and_set_team_cover_photo

> upload_and_set_team_cover_photo(team_id, file)
Upload a team cover photo

This updates the cover photo of the team. The cover photo must be a valid image file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **String** | The ID of the team to be updated. | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

