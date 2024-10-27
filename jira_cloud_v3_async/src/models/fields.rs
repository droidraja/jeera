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

/// Fields : Can contain multiple field values of following types depending on `type` key
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Fields {
    #[serde(rename="mandatoryField")]
    MandatoryFieldValue {
        /// If `true`, will try to retain original non-null issue field values on move.
        #[serde(rename = "retain", skip_serializing_if = "Option::is_none")]
        retain: Option<bool>,
        #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
    },
    #[serde(rename="mandatoryFieldForADF")]
    MandatoryFieldValueForAdf {
        /// If `true`, will try to retain original non-null issue field values on move.
        #[serde(rename = "retain", skip_serializing_if = "Option::is_none")]
        retain: Option<bool>,
        #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
        value: Option<serde_json::Value>,
    },
}

impl Default for Fields {
    fn default() -> Self {
        Self::MandatoryFieldValue {
            retain: Default::default(),
            value: Default::default(),
        }
        
    }
}

