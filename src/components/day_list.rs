use druid::widget::{
    Scroll, List, 
    Label, LineBreaking,
};
use druid::{WidgetExt};

use crate::types::State;

pub fn make_day_list() -> impl WidgetExt<State> {
    Scroll::new(
        List::new(|| {
            Label::dynamic(|data: &String, _env: &_| {
                data.to_string()
            })
            .with_line_break_mode(LineBreaking::WordWrap)
            .padding(10.0)
        })
    )
    .vertical()
    .lens(State::days)
}