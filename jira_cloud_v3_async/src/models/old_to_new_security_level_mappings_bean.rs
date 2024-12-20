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
pub struct OldToNewSecurityLevelMappingsBean {
    /// The new issue security level ID. Providing null will clear the assigned old level from issues.
    #[serde(rename = "newLevelId")]
    pub new_level_id: String,
    /// The old issue security level ID. Providing null will remap all issues without any assigned levels.
    #[serde(rename = "oldLevelId")]
    pub old_level_id: String,
}

impl OldToNewSecurityLevelMappingsBean {
    pub fn new(new_level_id: String, old_level_id: String) -> OldToNewSecurityLevelMappingsBean {
        OldToNewSecurityLevelMappingsBean {
            new_level_id,
            old_level_id,
        }
    }
}

