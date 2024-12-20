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
pub struct BulkOperationErrorResult {
    #[serde(rename = "elementErrors", skip_serializing_if = "Option::is_none")]
    pub element_errors: Option<Box<models::ErrorCollection>>,
    #[serde(rename = "failedElementNumber", skip_serializing_if = "Option::is_none")]
    pub failed_element_number: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl BulkOperationErrorResult {
    pub fn new() -> BulkOperationErrorResult {
        BulkOperationErrorResult {
            element_errors: None,
            failed_element_number: None,
            status: None,
        }
    }
}

