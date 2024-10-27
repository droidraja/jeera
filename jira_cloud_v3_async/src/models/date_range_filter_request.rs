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

/// DateRangeFilterRequest : List issues archived within a specified date range.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateRangeFilterRequest {
    /// List issues archived after a specified date, passed in the YYYY-MM-DD format.
    #[serde(rename = "dateAfter")]
    pub date_after: String,
    /// List issues archived before a specified date provided in the YYYY-MM-DD format.
    #[serde(rename = "dateBefore")]
    pub date_before: String,
}

impl DateRangeFilterRequest {
    /// List issues archived within a specified date range.
    pub fn new(date_after: String, date_before: String) -> DateRangeFilterRequest {
        DateRangeFilterRequest {
            date_after,
            date_before,
        }
    }
}
