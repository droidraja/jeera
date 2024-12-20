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

/// TargetStatus : Status mapping for statuses in source workflow to respective target status in target workflow.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetStatus {
    /// An object with the key as the ID of the target status and value with the list of the IDs of the current source statuses.
    #[serde(rename = "statuses")]
    pub statuses: std::collections::HashMap<String, Vec<String>>,
}

impl TargetStatus {
    /// Status mapping for statuses in source workflow to respective target status in target workflow.
    pub fn new(statuses: std::collections::HashMap<String, Vec<String>>) -> TargetStatus {
        TargetStatus {
            statuses,
        }
    }
}

