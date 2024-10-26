/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-e098eec8c0925855876f3d946f20a4b01cd69e3c
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// Version : Details about a project version.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// If the expand option `approvers` is used, returns a list containing the approvers for this version.
    #[serde(rename = "approvers", skip_serializing_if = "Option::is_none")]
    pub approvers: Option<Vec<models::VersionApprover>>,
    /// Indicates that the version is archived. Optional when creating or updating a version.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// The description of the version. Optional when creating or updating a version. The maximum size is 16,384 bytes.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If the expand option `driver` is used, returns the Atlassian account ID of the driver.
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Use [expand](em>#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:   *  `operations` Returns the list of operations available for this version.  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.  *  `driver` Returns the Atlassian account ID of the version driver.  *  `approvers` Returns a list containing approvers for this version.  Optional for create and update.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The ID of the version.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If the expand option `issuesstatus` is used, returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.
    #[serde(rename = "issuesStatusForFixVersion", skip_serializing_if = "Option::is_none")]
    pub issues_status_for_fix_version: Option<models::VersionIssuesStatus>,
    /// The URL of the self link to the version to which all unfixed issues are moved when a version is released. Not applicable when creating a version. Optional when updating a version.
    #[serde(rename = "moveUnfixedIssuesTo", skip_serializing_if = "Option::is_none")]
    pub move_unfixed_issues_to: Option<String>,
    /// The unique name of the version. Required when creating a version. Optional when updating a version. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If the expand option `operations` is used, returns the list of operations available for this version.
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<models::SimpleLink>>,
    /// Indicates that the version is overdue.
    #[serde(rename = "overdue", skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
    /// Deprecated. Use `projectId`.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The ID of the project to which this version is attached. Required when creating a version. Not applicable when updating a version.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The release date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Indicates that the version is released. If the version is released a request to release again is ignored. Not applicable when creating a version. Optional when updating a version.
    #[serde(rename = "released", skip_serializing_if = "Option::is_none")]
    pub released: Option<bool>,
    /// The URL of the version.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The start date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The date on which work on this version is expected to finish, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userReleaseDate", skip_serializing_if = "Option::is_none")]
    pub user_release_date: Option<String>,
    /// The date on which work on this version is expected to start, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userStartDate", skip_serializing_if = "Option::is_none")]
    pub user_start_date: Option<String>,
}

impl Version {
    /// Details about a project version.
    pub fn new() -> Version {
        Version {
            approvers: None,
            archived: None,
            description: None,
            driver: None,
            expand: None,
            id: None,
            issues_status_for_fix_version: None,
            move_unfixed_issues_to: None,
            name: None,
            operations: None,
            overdue: None,
            project: None,
            project_id: None,
            release_date: None,
            released: None,
            param_self: None,
            start_date: None,
            user_release_date: None,
            user_start_date: None,
        }
    }
}

