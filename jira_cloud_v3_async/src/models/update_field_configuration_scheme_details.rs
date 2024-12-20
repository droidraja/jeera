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

/// UpdateFieldConfigurationSchemeDetails : The details of the field configuration scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFieldConfigurationSchemeDetails {
    /// The description of the field configuration scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the field configuration scheme. The name must be unique.
    #[serde(rename = "name")]
    pub name: String,
}

impl UpdateFieldConfigurationSchemeDetails {
    /// The details of the field configuration scheme.
    pub fn new(name: String) -> UpdateFieldConfigurationSchemeDetails {
        UpdateFieldConfigurationSchemeDetails {
            description: None,
            name,
        }
    }
}

