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

/// Worklog : Details of a worklog.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Worklog {
    /// Details of the user who created the worklog.
    #[serde(rename = "author", skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<models::UserDetails>>,
    /// A comment about the worklog in [Atlassian Document Format](https://developer.atlassian.com/cloud/jira/platform/apis/document/structure/). Optional when creating or updating a worklog.
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<serde_json::Value>>,
    /// The datetime on which the worklog was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The ID of the worklog record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the issue this worklog is for.
    #[serde(rename = "issueId", skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    /// Details of properties for the worklog. Optional when creating or updating a worklog.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::EntityProperty>>,
    /// The URL of the worklog item.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The datetime on which the worklog effort was started. Required when creating a worklog. Optional when updating a worklog.
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    /// The time spent working on the issue as days (\\#d), hours (\\#h), or minutes (\\#m or \\#). Required when creating a worklog if `timeSpentSeconds` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpentSecond` is provided.
    #[serde(rename = "timeSpent", skip_serializing_if = "Option::is_none")]
    pub time_spent: Option<String>,
    /// The time in seconds spent working on the issue. Required when creating a worklog if `timeSpent` isn't provided. Optional when updating a worklog. Cannot be provided if `timeSpent` is provided.
    #[serde(rename = "timeSpentSeconds", skip_serializing_if = "Option::is_none")]
    pub time_spent_seconds: Option<i64>,
    /// Details of the user who last updated the worklog.
    #[serde(rename = "updateAuthor", skip_serializing_if = "Option::is_none")]
    pub update_author: Option<Box<models::UserDetails>>,
    /// The datetime on which the worklog was last updated.
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    /// Details about any restrictions in the visibility of the worklog. Optional when creating or updating a worklog.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<models::Visibility>,
}

impl Worklog {
    /// Details of a worklog.
    pub fn new() -> Worklog {
        Worklog {
            author: None,
            comment: None,
            created: None,
            id: None,
            issue_id: None,
            properties: None,
            param_self: None,
            started: None,
            time_spent: None,
            time_spent_seconds: None,
            update_author: None,
            updated: None,
            visibility: None,
        }
    }
}

