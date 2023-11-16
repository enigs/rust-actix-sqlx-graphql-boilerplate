use async_graphql::Context;
use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BearerToken(pub String);

impl Display for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl BearerToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }

    pub fn get(ctx: &Context<'_>) -> String {
        match ctx.data_opt::<Self>() {
            Some(token) => token.to_string(),
            None => String::new()
        }
    }
}