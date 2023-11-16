use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExpiredToken(pub String);

impl Display for ExpiredToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl ExpiredToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }
}