use async_graphql::{Object, Result};

#[derive(Default)]
pub struct VersionQuery;

#[Object]
impl VersionQuery {
    #[autometrics::autometrics]
    async fn display(&self) -> Result<String> {
        Ok(format!("Query Version: {}", env!("CARGO_PKG_VERSION")))
    }
}