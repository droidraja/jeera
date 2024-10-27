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

/// WorkflowCondition : The workflow transition rule conditions tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "nodeType")]
pub enum WorkflowCondition {
    #[serde(rename="simple")]
    Simple(Box<models::WorkflowSimpleCondition>),
    #[serde(rename="compound")]
    Compound(Box<models::WorkflowCompoundCondition>),
}

impl Default for WorkflowCondition {
    fn default() -> Self {
        Self::Simple(Default::default())
    }
}

/// The compound condition operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::And
    }
}
