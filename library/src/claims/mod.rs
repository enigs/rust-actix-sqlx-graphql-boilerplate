use async_graphql::{Context, Result};
use serde::{Serialize, Deserialize};

use crate::Errors;
use crate::prelude::{CustomRole, CustomStatus};
use crate::Response;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Claims<R, S> where R: CustomRole, S: CustomStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<R>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<S>,
}

impl <R, S> Claims <R, S> where R: CustomRole, S: CustomStatus  {
    pub fn get(ctx: &Context<'_>) -> Result<Self> {
        let response = Response::InternalServerError;
        match ctx.data_opt::<Self>() {
            Some(claims) => Ok(claims.clone()),
            None =>  Err(Errors::to(response, "Unable to parse claims from context"))
        }
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl<R, S> From<serde_json::Value> for Claims<R, S> where R: CustomRole, S: CustomStatus {
    fn from(value: serde_json::Value) -> Self {
        let aid = value.get("aid").and_then(|v| v.as_str()).map(|v| v.to_string());
        let sid = value.get("sid").and_then(|v| v.as_str()).map(|v| v.to_string());
        let role = value.get("role").and_then(|v| v.as_str()).map(|v| R::from_str(v));
        let status = value.get("status").and_then(|v| v.as_str()).map(|v| S::from_str(v));

        Self {
            aid,
            sid,
            role,
            status,
        }
    }
}