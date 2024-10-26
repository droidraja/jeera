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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PublicApiMembershipAddResponseErrorsInner {
    PublicApiAddMemberError(Box<models::PublicApiAddMemberError>),
    PublicApiMemberNotFoundError(Box<models::PublicApiMemberNotFoundError>),
    PublicApiRemoveMemberError(Box<models::PublicApiRemoveMemberError>),
}

impl Default for PublicApiMembershipAddResponseErrorsInner {
    fn default() -> Self {
        Self::PublicApiAddMemberError(Default::default())
    }
}

