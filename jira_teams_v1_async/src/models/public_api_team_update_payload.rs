/*
 * Teams Public API
 *
 * Teams Public API
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicApiTeamUpdatePayload {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl PublicApiTeamUpdatePayload {
    pub fn new() -> PublicApiTeamUpdatePayload {
        PublicApiTeamUpdatePayload {
            description: None,
            display_name: None,
        }
    }
}

