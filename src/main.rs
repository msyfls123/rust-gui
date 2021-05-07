#![allow(irrefutable_let_patterns)]

use druid::widget::{
    Button, Flex, Label, LineBreaking, EnvScope,
    ControllerHost, RawLabel, CrossAxisAlignment,
    TextBox,
};
use druid::{
    AppLauncher, LocalizedString, PlatformError,
    Widget, WidgetExt, WindowDesc, Env,
    MenuDesc, Color, theme,
};
use druid::im::{vector};

// use std::sync::mpsc::{Sender, channel, Receiver};
use tokio::sync::mpsc::{unbounded_channel};
use tokio::sync::{Mutex};
use std::sync::{Arc};

mod types;
mod utils;
mod helpers;
mod components;

use types::{State, rich_editor};
use helpers::event_handler::request_day;
use helpers::window_controller::WindowController;
use helpers::app_delegate::AppDelegater;
use helpers::{rich_editor as rich_editor_helpers};
use components::day_list::make_day_list;
use components::menu::{make_demo_menu, make_window_menu};

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    let (tx, rx) = unbounded_channel();

    let main_window = WindowDesc::new(ui_builder);
    let submenu = make_demo_menu();
    let menu = MenuDesc::new(LocalizedString::new("start"))
        .append(submenu.clone())
        .append(make_window_menu());
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
            env.set(theme::BUTTON_LIGHT, Color::WHITE);
            env.set(theme::BUTTON_DARK, Color::WHITE);
            env.set(theme::BACKGROUND_DARK, Color::GRAY);
            env.set(theme::BACKGROUND_LIGHT, Color::WHITE);
        })
        .delegate(AppDelegater {})
        .launch(State {
            day: 0_u32,
            concurrency: initial_concurrency,
            dispatch: tx.clone(),
            day_data: String::from(""),
            days: vector![],
            color_index: 0,
            rich_raw: rich_editor::INITIAL_RICH_TEXT.to_owned(),
            rich_text: rich_editor_helpers::generate_rich_data(rich_editor::INITIAL_RICH_TEXT),
        })
}

        
fn ui_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.day.into());
    let label = Label::new(text).padding(5.0);
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

    let editor = TextBox::multiline()
        .lens(State::rich_raw)
        .controller(rich_editor_helpers::RichEditorController)
        .expand_width()
        .padding(5.0);
    let rich_label = RawLabel::new().lens(State::rich_text).padding(5.0);
    let env_scoped_flex = EnvScope::new(
        |env: &mut Env, data: &State| {
            let options = [Color::TEAL, Color::AQUA, Color::NAVY, Color::MAROON];
            let len = options.len();
            env.set(theme::LABEL_COLOR, options[data.color_index % len].clone());
        },
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .with_child(label)
            .with_child(button)
            .with_default_spacer()
            .with_child(label2)
            .with_default_spacer()
            .with_child(label3)
            .with_child(rich_label)
            .with_default_spacer()
            .with_child(editor)
            .with_flex_child(make_day_list(), 1.0)
    );
    ControllerHost::new(env_scoped_flex, WindowController)
}
