use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgentProduct {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
}

impl UserAgentProduct {
    pub fn new() -> Self {
        Self {
            name: None,
            major: None,
            minor: None,
            patch: None,
        }
    }
}
