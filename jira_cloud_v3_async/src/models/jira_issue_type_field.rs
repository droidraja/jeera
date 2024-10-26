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
pub struct JiraIssueTypeField {
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
}

impl JiraIssueTypeField {
    pub fn new(issue_type_id: String) -> JiraIssueTypeField {
        JiraIssueTypeField {
            issue_type_id,
        }
    }
}

