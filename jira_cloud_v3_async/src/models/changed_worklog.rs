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

/// ChangedWorklog : Details of a changed worklog.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangedWorklog {
    /// Details of properties associated with the change.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<models::EntityProperty>>,
    /// The datetime of the change.
    #[serde(rename = "updatedTime", skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<i64>,
    /// The ID of the worklog.
    #[serde(rename = "worklogId", skip_serializing_if = "Option::is_none")]
    pub worklog_id: Option<i64>,
}

impl ChangedWorklog {
    /// Details of a changed worklog.
    pub fn new() -> ChangedWorklog {
        ChangedWorklog {
            properties: None,
            updated_time: None,
            worklog_id: None,
        }
    }
}

