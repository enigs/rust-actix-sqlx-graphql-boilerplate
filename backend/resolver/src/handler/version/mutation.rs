use async_graphql::{Object, Result};

#[derive(Default)]
pub struct VersionMutation;

#[Object]
impl VersionMutation {
    #[autometrics::autometrics]
    async fn display(&self) -> Result<String> {
        Ok(format!("Mutation Version: {}", env!("CARGO_PKG_VERSION")))
    }
}
