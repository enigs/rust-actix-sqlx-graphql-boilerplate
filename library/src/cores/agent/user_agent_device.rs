use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgentDevice {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
}

impl UserAgentDevice {
    pub fn new() -> Self {
        Self {
            name: None,
            brand: None,
            model: None
        }
    }
}
