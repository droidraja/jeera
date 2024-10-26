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

/// WorkflowRuleConfiguration : The configuration of the rule.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRuleConfiguration {
    /// The ID of the rule.
    #[serde(rename = "id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub id: Option<Option<String>>,
    /// The parameters related to the rule.
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    /// The rule key of the rule.
    #[serde(rename = "ruleKey")]
    pub rule_key: String,
}

impl WorkflowRuleConfiguration {
    /// The configuration of the rule.
    pub fn new(rule_key: String) -> WorkflowRuleConfiguration {
        WorkflowRuleConfiguration {
            id: None,
            parameters: None,
            rule_key,
        }
    }
}

