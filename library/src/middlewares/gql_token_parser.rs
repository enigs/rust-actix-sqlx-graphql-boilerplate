use async_graphql::{ Request, ServerResult };
use async_graphql::async_trait::async_trait;
use async_graphql::extensions::{ Extension, ExtensionContext, ExtensionFactory, NextPrepareRequest };
use std::sync::{ Arc, RwLock };

use crate::ciphers::Cipher;
use crate::{BearerToken, ExpiredToken, InvalidToken};
use crate::Paseto;
use crate::prelude::{CustomRole, CustomStatus};

pub struct GqlTokenParser<R, S> where R: CustomRole, S: CustomStatus {
    role: R,
    status: S,
}

impl <R, S> GqlTokenParser<R, S> where R: CustomRole, S: CustomStatus {
    pub fn new(role: R, status: S) -> Self {
        Self {
            role,
            status,
        }
    }
}

impl<R, S> ExtensionFactory for GqlTokenParser<R, S> where R: CustomRole, S: CustomStatus {
    fn create(&self) -> Arc<dyn Extension> {
        let _role = self.role.clone();
        let _status = self.status.clone();

        Arc::new(GqlTokenParserExtension::new(_role, _status))
    }
}

pub struct GqlTokenParserExtension<R, S> where R: CustomRole, S: CustomStatus {
    _role: R,
    _status: S,
}

impl <R, S> GqlTokenParserExtension<R, S> where R: CustomRole, S: CustomStatus {
    pub fn new(_role: R, _status: S) -> Self {
        Self {
            _role,
            _status,
        }
    }
}

#[async_trait]
impl<R, S> Extension for GqlTokenParserExtension<R, S> where R: CustomRole, S: CustomStatus  {
    async fn prepare_request(
        &self,
        ctx: &ExtensionContext<'_>,
        request: Request,
        next: NextPrepareRequest<'_>,
    ) -> ServerResult<Request> {
        let token = Cipher::from(match ctx.data_opt::<BearerToken>() {
            Some(token) => token.to_string(),
            None => String::new()
        }).decrypt()
            .unwrap_or_default()
            .b64encode()
            .unwrap_or_default();

        let master = std::env::var("MASTER_KEY")
            .expect("MASTER_KEY is not set");

        let role = match token == master && !token.is_empty() && !master.is_empty() {
            true => R::get_controller(),
            false => R::get_guest()
        };

        let mut request = request.data(role.clone());

        if role.is_guest() {
            if let Some(paseto) = ctx.data_opt::<Arc<RwLock<Paseto>>>() {
                if let Ok(paseto) = paseto.try_read() {
                    let result: anyhow::Result<crate::Claims<R, S>> = paseto
                        .clone()
                        .validate_access_token(&token);

                    match result {
                        Ok(claims) => if !claims.is_empty() {
                            request = request.data(claims.clone().role.unwrap_or_default()).data(claims);
                        },
                        Err(error) => match error
                            .to_string()
                            .to_lowercase()
                            .as_str()
                        {
                            "your authentication token has expired" |
                            "your refresh token has expired" => request = request.data(role).data(ExpiredToken::default()),
                            _ => request = request.data(role).data(InvalidToken::default())
                        }
                    }
                }
            }
        }

        next.run(ctx, request).await
    }
}