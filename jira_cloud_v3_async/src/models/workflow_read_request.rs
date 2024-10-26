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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowReadRequest {
    /// The list of projects and issue types to query.
    #[serde(rename = "projectAndIssueTypes", skip_serializing_if = "Option::is_none")]
    pub project_and_issue_types: Option<Vec<models::ProjectAndIssueTypePair>>,
    /// The list of workflow IDs to query.
    #[serde(rename = "workflowIds", skip_serializing_if = "Option::is_none")]
    pub workflow_ids: Option<Vec<String>>,
    /// The list of workflow names to query.
    #[serde(rename = "workflowNames", skip_serializing_if = "Option::is_none")]
    pub workflow_names: Option<Vec<String>>,
}

impl WorkflowReadRequest {
    pub fn new() -> WorkflowReadRequest {
        WorkflowReadRequest {
            project_and_issue_types: None,
            workflow_ids: None,
            workflow_names: None,
        }
    }
}

