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

/// ParsedJqlQuery : Details of a parsed JQL query.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParsedJqlQuery {
    /// The list of syntax or validation errors.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// The JQL query that was parsed and validated.
    #[serde(rename = "query")]
    pub query: String,
    /// The syntax tree of the query. Empty if the query was invalid.
    #[serde(rename = "structure", skip_serializing_if = "Option::is_none")]
    pub structure: Option<Box<models::JqlQuery>>,
}

impl ParsedJqlQuery {
    /// Details of a parsed JQL query.
    pub fn new(query: String) -> ParsedJqlQuery {
        ParsedJqlQuery {
            errors: None,
            query,
            structure: None,
        }
    }
}

