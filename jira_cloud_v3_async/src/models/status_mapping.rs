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

/// StatusMapping : Details about the mapping from a status to a new status for an issue type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusMapping {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the new status.
    #[serde(rename = "newStatusId")]
    pub new_status_id: String,
    /// The ID of the status.
    #[serde(rename = "statusId")]
    pub status_id: String,
}

impl StatusMapping {
    /// Details about the mapping from a status to a new status for an issue type.
    pub fn new(issue_type_id: String, new_status_id: String, status_id: String) -> StatusMapping {
        StatusMapping {
            issue_type_id,
            new_status_id,
            status_id,
        }
    }
}
