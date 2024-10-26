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

/// PermissionGrants : List of permission grants.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionGrants {
    /// Expand options that include additional permission grant details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// Permission grants list.
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<models::PermissionGrant>>,
}

impl PermissionGrants {
    /// List of permission grants.
    pub fn new() -> PermissionGrants {
        PermissionGrants {
            expand: None,
            permissions: None,
        }
    }
}

