use serde::{Serialize, Deserialize};

use crate::Errors;
use crate::Response;

mod user_agent_cpu;
mod user_agent_device;
mod user_agent_engine;
mod user_agent_os;
mod user_agent_product;

use user_agent_cpu::UserAgentCPU;
use user_agent_device::UserAgentDevice;
use user_agent_engine::UserAgentEngine;
use user_agent_os::UserAgentOS;
use user_agent_product::UserAgentProduct;

// Create user agent object
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserAgent {
    pub ip: Option<String>,
    pub product: UserAgentProduct,
    pub os: UserAgentOS,
    pub device: UserAgentDevice,
    pub cpu: UserAgentCPU,
    pub engine: UserAgentEngine,
}

// Create implementation for UserAgent
impl UserAgent {
    // Creates a new user agent
    pub fn new() -> Self {
        Self {
            ip: None,
            product: UserAgentProduct::new(),
            os: UserAgentOS::new(),
            device: UserAgentDevice::new(),
            cpu: UserAgentCPU::new(),
            engine: UserAgentEngine::new(),
        }
    }

    // Convert self to json value
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self.clone()).unwrap()
    }

    /// Parse from async graphql context
    pub fn parse<'a>(ctx: &'a async_graphql::Context<'_>) -> async_graphql::Result<&'a Self> {
        let response = Response::InternalServerError;
        let error = "Unable to initialize user agent configuration";

        match ctx.data_opt::<UserAgent>() {
            Some(settings) => Ok(settings),
            None =>  Err(Errors::to(response, error))
        }
    }

    /// Parse from async graphql context
    pub fn get(ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Self> {
        match Self::parse(ctx) {
            Ok(settings) => Ok(settings.clone()),
            Err(error) => Err(error)
        }
    }
}