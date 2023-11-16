use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgentOS {
    pub name: Option<String>,
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub patch_minor: Option<String>
}

impl UserAgentOS {
    pub fn new() -> Self {
        Self {
            name: None,
            major: None,
            minor: None,
            patch: None,
            patch_minor: None,
        }
    }
}
