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

/// StatusMetadata : The details of the statuses in the associated workflows.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMetadata {
    /// The category of the status.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// The ID of the status.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl StatusMetadata {
    /// The details of the statuses in the associated workflows.
    pub fn new() -> StatusMetadata {
        StatusMetadata {
            category: None,
            id: None,
            name: None,
        }
    }
}
/// The category of the status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "TODO")]
    Todo,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "DONE")]
    Done,
}

impl Default for Category {
    fn default() -> Category {
        Self::Todo
    }
}

