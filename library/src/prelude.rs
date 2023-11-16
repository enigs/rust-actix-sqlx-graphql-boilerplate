use serde::Serialize;
use std::fmt::Debug;

pub trait CustomRole: Default + Debug + Clone + PartialEq + Serialize + Sync + Send + 'static {
    fn get(ctx: &async_graphql::Context<'_>) -> Self;

    fn get_controller() -> Self;

    fn get_admin() -> Self;

    fn get_guest() -> Self;

    fn is_controller(&self) -> bool {
        *self == Self::get_controller()
    }

    fn is_admin(&self) -> bool {
        *self == Self::get_admin()
    }

    fn is_guest(&self) -> bool {
        *self == Self::get_guest()
    }

    fn from_str(s: &str) -> Self;

    fn to_string(&self) -> String;
}

pub trait CustomStatus: Default + Clone + PartialEq + Serialize + Sync + Send + 'static {
    fn from_str(s: &str) -> Self;

    fn to_string(&self) -> String;
}