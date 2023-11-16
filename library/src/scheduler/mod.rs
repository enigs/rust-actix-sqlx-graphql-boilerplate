use actix::prelude::*;
use chrono::Local;
use cron::Schedule;
use std::{str::FromStr, time::Duration};
use std::sync::Arc;

use crate::Core;

fn duration_timer<T: Into<String>>(duration: T) -> Duration {
    let bindings = duration.into();
    let cron_schedule = Schedule::from_str(&bindings).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);

    duration_until.to_std().unwrap()
}

#[derive(Default, Clone)]
pub struct Scheduler {
    pub core: Option<Arc<Core>>,
    pub duration: String,
    pub func: Option<fn(Arc<Core>)>
}

impl Actor for Scheduler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("=== Cron started running at {} ===", Local::now().format("%b %d, %Y - %I:%M%p"));

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("=== Cron stopped running at {} ===", Local::now().format("%b %d, %Y - %I:%M%p"));
    }
}

impl Scheduler {
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_core(&mut self, core: &Arc<Core>) -> &mut Self {
        self.core = Some(Arc::clone(core));
        self
    }

    pub fn set_duration(&mut self, duration: &str) -> &mut Self {
        self.duration = duration.to_string();
        self
    }

    pub fn set_func(&mut self, func: fn(Arc<Core>)) -> &mut Self {
        self.func = Some(func);
        self
    }

    fn schedule_task(&self, ctx: &mut Context<Self>) {
        if let Some(core) = &self.core {
            if let Some(func) = self.func {
                func(Arc::clone(core));
            }
        }

        println!("Task {}: ", Local::now().format("%b %d, %Y - %I:%M%p"));

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}