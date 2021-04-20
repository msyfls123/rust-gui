#![allow(irrefutable_let_patterns)]

use std::time;
use druid::widget::{Button, Flex, Label };
use druid::{
    AppLauncher, LocalizedString, PlatformError,
    Widget, WidgetExt, WindowDesc, Data, Lens, Target,
    Selector, AppDelegate, DelegateCtx, Handled, Command, Env,
};
use reqwest::Error;
use serde::{Deserialize};
use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Date {
    Int(u32),
    String(String),
}

#[derive(Deserialize, Debug, Clone)]
struct Day {
    date: Date,
    content: String,
    suggestion: String,
}

const DAY_DATA: Selector<String> = Selector::new("day_data");

#[derive(Clone, Data, Lens)]
struct State {
    day: u32,
    #[data(ignore)]
    dispatch: Sender<u32>,
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
        } else {
            Handled::No
        }
    }
}

// #[tokio::main]
fn main() -> Result<(), PlatformError> {
    let (tx, rx) = channel();
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();
    request_day(rx, event_sink);
    launcher.use_simple_logger()
        .delegate(Delegate {})
        .launch(State {
            day: data,
            dispatch: tx.clone(),
            day_data: String::from(""),
        })

}

fn request_day(rx: Receiver<u32>, event_sink: druid::ExtEventSink) {
    thread::spawn(move || {
        while let res = rx.recv() {
            match res {
                Ok(day) => {
                    println!("{}", day);
                    let res = get_data(day).unwrap();
                    thread::sleep(time::Duration::from_millis(1000));
                    let day_data = format!("{:?}", res);
                    event_sink
                        .submit_command(DAY_DATA, day_data, Target::Auto)
                        .expect("uhhh");
                },
                Err(err) => eprintln!("{:?}", err),
            }
        }
    });
}

fn ui_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.day.into());
    let label = Label::new(text).padding(5.0).center();
    let label2 = Label::new(|data: &State, _env: &_| format!("{}", data.day_data)).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut State, _env| {
            data.day += 1;
            if data.dispatch.send(data.day).is_err() {
                eprintln!("dispatch err");
            };
        })
        .padding(5.0);

    Flex::column().with_child(label).with_child(button).with_child(label2)
}

fn get_data(day: u32) -> Result<Day, Error> {
    let url = format!("https://day.ebichu.cc/api/{}", day);
    let response = reqwest::blocking::get(&url)?;
    let day: Day = response.json()?;
    Ok(day)
}
