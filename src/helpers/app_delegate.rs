use druid::{
    AppDelegate, DelegateCtx, Target, Handled,
    Command, Env,
};

use crate::types::{State};
use crate::types::selector::{
    DAY_DATA,
    MENU_COUNT_ACTION,
    CONCURRENCY_COUNT,
};

pub struct AppDelegater;

impl AppDelegate<State> for AppDelegater {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut State,
        _env: &Env,
    ) -> Handled {
        if let Some(day) = cmd.get(DAY_DATA) {
            data.day_data = day.to_string();
            data.days.push_back(day.to_string());
            Handled::Yes
        } else if let Some(&concurrency) = cmd.get(CONCURRENCY_COUNT) {
            data.concurrency = concurrency;
            Handled::Yes
        } else if let Some(&index) = cmd.get(MENU_COUNT_ACTION) {
            data.color_index = index;
            Handled::Yes
        } else {
            Handled::No
        }
    }
}