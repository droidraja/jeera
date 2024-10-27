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

/// CreateWorkflowTransitionRule : A workflow transition rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateWorkflowTransitionRule {
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The type of the transition rule.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CreateWorkflowTransitionRule {
    /// A workflow transition rule.
    pub fn new(r#type: String) -> CreateWorkflowTransitionRule {
        CreateWorkflowTransitionRule {
            configuration: None,
            r#type,
        }
    }
}
