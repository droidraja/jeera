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

/// Visibility : The group or role to which this item is visible.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Visibility {
    /// The ID of the group or the name of the role that visibility of this item is restricted to.
    #[serde(rename = "identifier", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Option<String>>,
    /// Whether visibility of this item is restricted to a group or role.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The name of the group or role that visibility of this item is restricted to. Please note that the name of a group is mutable, to reliably identify a group use `identifier`.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl Visibility {
    /// The group or role to which this item is visible.
    pub fn new() -> Visibility {
        Visibility {
            identifier: None,
            r#type: None,
            value: None,
        }
    }
}
/// Whether visibility of this item is restricted to a group or role.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "role")]
    Role,
}

impl Default for Type {
    fn default() -> Type {
        Self::Group
    }
}

