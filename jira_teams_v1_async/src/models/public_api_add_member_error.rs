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
pub struct PublicApiAddMemberError {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl PublicApiAddMemberError {
    pub fn new(account_id: String, code: String, message: String) -> PublicApiAddMemberError {
        PublicApiAddMemberError {
            account_id,
            code,
            message,
        }
    }
}

