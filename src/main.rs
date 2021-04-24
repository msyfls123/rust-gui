#![allow(irrefutable_let_patterns)]

use druid::widget::{Button, Flex, Label, LineBreaking};
use druid::{
    AppLauncher, LocalizedString, PlatformError,
    Widget, WidgetExt, WindowDesc, Data, Lens, Target,
    AppDelegate, DelegateCtx, Handled, Command, Env,
    MenuDesc, MenuItem, Color,
};
// use std::sync::mpsc::{Sender, channel, Receiver};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::sync::{Mutex};
use std::sync::{Arc};

mod types;
mod utils;
mod helpers;

use types::selector::{
    DAY_DATA,
    MENU_COUNT_ACTION,
    CONCURRENCY_COUNT,
};
use helpers::event_handler::request_day;

#[derive(Debug, Clone, Data, Lens)]
struct State {
    day: u32,
    concurrency: u32,
    #[data(ignore)]
    dispatch: UnboundedSender<u32>,
    day_data: String,
}

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
        } else {
            Handled::No
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    let (tx, rx) = unbounded_channel();

    let main_window = WindowDesc::new(ui_builder);
    let submenu = MenuDesc::new(LocalizedString::new("hello2")).append_iter(|| (0..4).map(|i| {
        MenuItem::new(
            LocalizedString::new("hello-counter").with_arg("count", move |_, _| i.into()),
            Command::new(MENU_COUNT_ACTION, i, Target::Auto),
        )
    }));
    let menu = MenuDesc::new(LocalizedString::new("hello"))
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
        .delegate(Delegate {})
        .launch(State {
            day: 0_u32,
            concurrency: initial_concurrency,
            dispatch: tx.clone(),
            day_data: String::from(""),
        })
}

        
fn ui_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.day.into());
    let label = Label::new(text).padding(5.0).center();
    let label2 = Label::new(|data: &State, _env: &_| format!("{}", data.day_data))
        .with_line_break_mode(LineBreaking::WordWrap)
        .with_text_color(Color::rgb8(0x39, 0xff, 0xab))
        .padding(5.0);
    let label3 = Label::new(|data: &State, _env: &_| format!("Concurrency: {}", data.concurrency)).padding(5.0);
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut State, _env| {
            data.day += 1;
            eprintln!("emmm {}", data.day);
            if data.dispatch.send(data.day).is_err() {
                eprintln!("emmm");
            };
        })
        .padding(5.0);

    Flex::column()
        .with_child(label)
        .with_child(button)
        .with_default_spacer()
        .with_child(label2)
        .with_default_spacer()
        .with_child(label3)
}
