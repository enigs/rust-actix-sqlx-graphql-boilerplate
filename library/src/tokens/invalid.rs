use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvalidToken(pub String);

impl Display for InvalidToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

impl InvalidToken {
    pub fn new<T>(token: T) -> Self
        where T: ToString
    {
        Self(token.to_string())
    }
}