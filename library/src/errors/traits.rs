use crate::Errors;

impl From<&str> for Errors {
    fn from(message: &str) -> Self {
        Errors {
            message: Some(message.to_string()),
            ..Default::default()
        }
    }
}

impl From<String> for Errors {
    fn from(message: String) -> Self {
        Self::from(message.as_str())
    }
}