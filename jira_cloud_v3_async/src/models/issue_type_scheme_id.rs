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

/// IssueTypeSchemeId : The ID of an issue type scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeSchemeId {
    /// The ID of the issue type scheme.
    #[serde(rename = "issueTypeSchemeId")]
    pub issue_type_scheme_id: String,
}

impl IssueTypeSchemeId {
    /// The ID of an issue type scheme.
    pub fn new(issue_type_scheme_id: String) -> IssueTypeSchemeId {
        IssueTypeSchemeId {
            issue_type_scheme_id,
        }
    }
}
