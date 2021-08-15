use druid::widget::{Controller};
use druid::{Widget, LifeCycle, LifeCycleCtx};
use crate::types::{State};
use std::time::{SystemTime};

pub struct TimingController;

pub fn get_timestamp() -> u128 {
  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}

impl <W: Widget<State>> Controller<State, W> for TimingController {
    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx<'_, '_>,
        event: &LifeCycle,
        data: &State,
        env: &druid::Env
    ) {
      
      match event {
        LifeCycle::Size(_) => {
          println!("Create window cost: {}", get_timestamp() - data.recent_open_win_time);
        },
        _ => {},
      }
      child.lifecycle(ctx, event, data, env);
    }

    fn event(
      &mut self,
      child: &mut W,
      ctx: &mut druid::EventCtx<'_, '_>,
      event: &druid::Event,
      data: &mut State,
      env: &druid::Env
  ) {
    // println!("{:?}", event);
    child.event(ctx, event, data, env);
  }
}
