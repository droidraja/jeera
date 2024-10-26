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

/// IssueTypeIssueCreateMetadata : Details of the issue creation metadata for an issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeIssueCreateMetadata {
    /// The ID of the issue type's avatar.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the issue type.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique ID for next-gen projects.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    /// Expand options that include additional issue type metadata details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of the fields available when creating an issue for the issue type.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, models::FieldMetadata>>,
    /// Hierarchy level of the issue type.
    #[serde(rename = "hierarchyLevel", skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i32>,
    /// The URL of the issue type's avatar.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the issue type.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the issue type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Details of the next-gen projects the issue type is available in.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<models::Scope>,
    /// The URL of these issue type details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Whether this issue type is used to create subtasks.
    #[serde(rename = "subtask", skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}

impl IssueTypeIssueCreateMetadata {
    /// Details of the issue creation metadata for an issue type.
    pub fn new() -> IssueTypeIssueCreateMetadata {
        IssueTypeIssueCreateMetadata {
            avatar_id: None,
            description: None,
            entity_id: None,
            expand: None,
            fields: None,
            hierarchy_level: None,
            icon_url: None,
            id: None,
            name: None,
            scope: None,
            param_self: None,
            subtask: None,
        }
    }
}

