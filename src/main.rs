#![allow(irrefutable_let_patterns)]

use druid::widget::{
    Button, Flex, Label, LineBreaking, EnvScope,
    Controller, ControllerHost,
};
use druid::{
    AppLauncher, LocalizedString, PlatformError,
    Widget, WidgetExt, WindowDesc, Target,
    AppDelegate, DelegateCtx, Handled, Command, Env,
    MenuDesc, Color, theme, Event, ContextMenu,
};
// use std::sync::mpsc::{Sender, channel, Receiver};
use tokio::sync::mpsc::{unbounded_channel};
use tokio::sync::{Mutex};
use std::sync::{Arc};

mod types;
mod utils;
mod helpers;
mod components;

use types::State;
use types::selector::{
    DAY_DATA,
    MENU_COUNT_ACTION,
    CONCURRENCY_COUNT,
};
use helpers::event_handler::request_day;
use components::menu::{make_demo_menu};


struct Delegate;

impl AppDelegate<State> for Delegate {
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

struct WindowContextMenuController;

impl <W: Widget<State>> Controller<State, W> for WindowContextMenuController {
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

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    let (tx, rx) = unbounded_channel();

    let main_window = WindowDesc::new(ui_builder);
    let submenu = make_demo_menu();
    let menu = MenuDesc::new(LocalizedString::new("start"))
        .append(submenu.clone())
        .append(submenu.clone());
    let main_window = main_window.menu(menu);
    let launcher = AppLauncher::with_window(main_window);

    let arc_rx = Arc::new(Mutex::new(rx));
    let event_sink = launcher.get_external_handle();
    let arc_event_sink = Arc::new(event_sink);
    let initial_concurrency = 0;
    request_day(initial_concurrency, arc_rx, Arc::clone(&arc_event_sink));

    launcher.use_simple_logger()
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::LABEL_COLOR, Color::AQUA);
        })
        .delegate(Delegate {})
        .launch(State {
            day: 0_u32,
            concurrency: initial_concurrency,
            dispatch: tx.clone(),
            day_data: String::from(""),
            color_index: 0,
        })
}

        
fn ui_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.day.into());
    let label = Label::new(text).padding(5.0).center();
    let label2 = Label::new(|data: &State, _env: &_| format!("{}", data.day_data))
        .with_line_break_mode(LineBreaking::WordWrap)
        .with_text_color(Color::rgb8(0x39, 0x9c, 0xab))
        .padding(5.0);
    let text2 = LocalizedString::new("concurrency").with_arg("count", |data: &State, _env| data.concurrency.into());
    let label3 = Label::new(text2).padding(5.0);
    let button_text = LocalizedString::<State>::new("increment");
    let button = Button::new(button_text)
        .on_click(|_ctx, data: &mut State, _env| {
            data.day += 1;
            eprintln!("emmm {}", data.day);
            if data.dispatch.send(data.day).is_err() {
                eprintln!("emmm");
            };
        })
        .padding(5.0);
    let env_scoped_flex = EnvScope::new(
        |env: &mut Env, data: &State| {
            let options = [Color::TEAL, Color::AQUA, Color::NAVY, Color::MAROON];
            let len = options.len();
            env.set(theme::LABEL_COLOR, options[data.color_index % len].clone());
        },
        Flex::column()
            .with_child(label)
            .with_child(button)
            .with_default_spacer()
            .with_child(label2)
            .with_default_spacer()
            .with_child(label3)
    );
    ControllerHost::new(env_scoped_flex, WindowContextMenuController)
}
