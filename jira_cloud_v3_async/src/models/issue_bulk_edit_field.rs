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
pub struct IssueBulkEditField {
    /// Description of the field.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of options related to the field, applicable in contexts where multiple selections are allowed.
    #[serde(rename = "fieldOptions", skip_serializing_if = "Option::is_none")]
    pub field_options: Option<Vec<serde_json::Value>>,
    /// The unique ID of the field.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates whether the field is mandatory for the operation.
    #[serde(rename = "isRequired", skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    /// Specifies supported actions (like add, replace, remove) on multi-select fields via an enum.
    #[serde(rename = "multiSelectFieldOptions", skip_serializing_if = "Option::is_none")]
    pub multi_select_field_options: Option<Vec<MultiSelectFieldOptions>>,
    /// The display name of the field.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A URL to fetch additional data for the field
    #[serde(rename = "searchUrl", skip_serializing_if = "Option::is_none")]
    pub search_url: Option<String>,
    /// The type of the field.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A message indicating why the field is unavailable for editing.
    #[serde(rename = "unavailableMessage", skip_serializing_if = "Option::is_none")]
    pub unavailable_message: Option<String>,
}

impl IssueBulkEditField {
    pub fn new() -> IssueBulkEditField {
        IssueBulkEditField {
            description: None,
            field_options: None,
            id: None,
            is_required: None,
            multi_select_field_options: None,
            name: None,
            search_url: None,
            r#type: None,
            unavailable_message: None,
        }
    }
}
/// Specifies supported actions (like add, replace, remove) on multi-select fields via an enum.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MultiSelectFieldOptions {
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "REMOVE_ALL")]
    RemoveAll,
}

impl Default for MultiSelectFieldOptions {
    fn default() -> MultiSelectFieldOptions {
        Self::Add
    }
}
