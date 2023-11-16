use async_graphql::{ Context, Guard as AsyncGQLGuard, Result };

use crate::Claims;
use crate::{Errors, Response};
use crate::{ExpiredToken, InvalidToken};
use crate::prelude::{CustomRole, CustomStatus};

const EXPIRED: &str = "We're sorry, but your authentication token has expired. Please sign in again to continue.";
const FORBIDDEN: &str = "Sorry, you do not have the required permissions to perform this action.";

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Guard<R, S> where R: CustomRole, S: CustomStatus {
    authentication: bool,
    role: Option<Vec<R>>,
    status: Option<Vec<S>>,
}

#[async_graphql::async_trait::async_trait]
impl<R, S> AsyncGQLGuard for Guard<R, S> where R: CustomRole, S: CustomStatus {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let role: R = match ctx.data_opt::<R>() {
            Some(role) => role.clone(),
            None => R::get_guest(),
        };

        let is_invalid = ctx.data_opt::<InvalidToken>().is_some();
        let is_expired = ctx.data_opt::<ExpiredToken>().is_some();

        let roles = match self.role.clone() {
            None => vec![],
            Some(roles) => roles
        };

        let claims = match ctx.data_opt::<Claims<R,S>>() {
            Some(claims) => claims.clone(),
            None => Claims::default()
        };

        if !self.authentication && roles.contains(&role) {
            return Ok(());
        }

        if self.authentication && !claims.is_empty()
            && !is_expired && !is_invalid {
            let role = claims.role
                .unwrap_or_default();

            if roles.contains(&role) {
                return Ok(());
            }
        }

        if self.authentication && is_expired {
            return Err(Errors::to(Response::Unauthorized, EXPIRED));
        }

        Err(Errors::to(Response::Forbidden, FORBIDDEN))
    }
}

impl <R,S> Guard<R, S> where R: CustomRole, S: CustomStatus {
    pub fn role(role: Vec<R>) -> Self {
        let role = match role.is_empty() {
            true => None,
            false => Some(role)
        };

        Self {
            role,
            ..Default::default()
        }
    }

    pub fn controller() -> Self {
        Self::role(vec![R::get_controller()])
    }

    pub fn is_controller(ctx: &Context) -> bool {
        R::get(ctx).is_controller()
    }

    pub fn is_admin(ctx: &Context) -> bool {
        R::get(ctx).is_admin()
    }

    pub fn is_guest(ctx: &Context) -> bool {
        R::get(ctx).is_guest()
    }
}