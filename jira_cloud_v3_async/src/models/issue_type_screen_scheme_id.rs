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

/// IssueTypeScreenSchemeId : The ID of an issue type screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeId {
    /// The ID of the issue type screen scheme.
    #[serde(rename = "id")]
    pub id: String,
}

impl IssueTypeScreenSchemeId {
    /// The ID of an issue type screen scheme.
    pub fn new(id: String) -> IssueTypeScreenSchemeId {
        IssueTypeScreenSchemeId {
            id,
        }
    }
}

