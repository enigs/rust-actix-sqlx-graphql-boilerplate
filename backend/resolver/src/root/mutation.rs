use async_graphql::Object;

use model::Guard;

#[derive(Default)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    #[graphql(visible = "Guard::is_controller", guard = "Guard::controller()")]
    async fn setup(&self) -> crate::SetupMutation {
        crate::SetupMutation
    }

    /// Temporarily assign this resolver in order for the playground to run properly.
    /// Remove if you have root mutations that does not contain any guards.
    async fn version(&self) -> crate::VersionMutation {
        crate::VersionMutation
    }
}