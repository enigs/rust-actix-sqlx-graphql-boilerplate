use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgentCPU {
    pub architecture: Option<String>,
}

impl UserAgentCPU {
    pub fn new() -> Self {
        Self {
            architecture: None,
        }
    }
}
