use async_graphql::Object;

use model::Guard;

#[derive(Default)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    #[graphql(visible = "Guard::is_controller", guard = "Guard::controller()")]
    async fn setup(&self) -> crate::SetupQuery {
        crate::SetupQuery
    }

    /// Temporarily assign this resolver in order for the playground to run properly.
    /// Remove if you have root queries that does not contain any guards.
    async fn version(&self) -> crate::VersionQuery {
        crate::VersionQuery
    }
}