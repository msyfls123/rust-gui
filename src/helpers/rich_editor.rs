use druid::widget::{Controller};
use druid::{
    KeyOrValue, Color, Widget,
    EventCtx, Event, Env, Data,
};
use druid::text::{Attribute, RichText};

use crate::types::{State};

pub fn generate_rich_data(text: &str) -> RichText {
    let attr = Attribute::TextColor(KeyOrValue::Concrete(Color::PURPLE));
    RichText::new(text.into())
        .with_attribute(6..=10, attr)
}

/// A controller that rebuilds the preview when edits occur
pub struct RichEditorController;

impl<W: Widget<State>> Controller<State, W> for RichEditorController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut State,
        env: &Env,
    ) {
        let pre_data = data.rich_raw.to_owned();
        child.event(ctx, event, data, env);
        if !data.rich_raw.same(&pre_data) {
            data.rich_text = generate_rich_data(&data.rich_raw);
        }
    }
}