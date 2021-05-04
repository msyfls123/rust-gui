use druid::widget::{Controller};
use druid::{Widget, Event, ContextMenu};
use crate::components::menu::make_demo_menu;
use crate::types::{State};

pub struct WindowController;

impl <W: Widget<State>> Controller<State, W> for WindowController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut druid::EventCtx<'_, '_>,
        event: &druid::Event,
        data: &mut State,
        env: &druid::Env
    ) {
        match event {
            Event::MouseDown(ref mouse) if mouse.button.is_right() => {
                let context_menu = ContextMenu::new(make_demo_menu(), mouse.pos);
                ctx.show_context_menu(context_menu);
            },
            _ => child.event(ctx, event, data, env),
        }
    }
}