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

/// WorkflowId : Properties that identify a workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowId {
    /// Whether the workflow is in the draft state.
    #[serde(rename = "draft")]
    pub draft: bool,
    /// The name of the workflow.
    #[serde(rename = "name")]
    pub name: String,
}

impl WorkflowId {
    /// Properties that identify a workflow.
    pub fn new(draft: bool, name: String) -> WorkflowId {
        WorkflowId {
            draft,
            name,
        }
    }
}

