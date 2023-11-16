use async_graphql::Context;

use library::Guard as GuardLib;

use crate::{Role, Status};

pub struct Guard;

impl Guard {
    pub fn controller() -> GuardLib<Role, Status> {
        GuardLib::<Role, Status>::role(vec![Role::Controller])
    }

    pub fn is_controller(ctx: &Context) -> bool {
        GuardLib::<Role, Status>::is_controller(ctx)
    }

    pub fn is_admin(ctx: &Context) -> bool {
        GuardLib::<Role, Status>::is_admin(ctx)
    }

    pub fn is_guest(ctx: &Context) -> bool {
        GuardLib::<Role, Status>::is_guest(ctx)
    }
}