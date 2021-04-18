use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data, Lens};
use reqwest::Error;
use serde::{Deserialize};
use std::sync::mpsc::{Sender, channel};
use std::thread;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Date {
    Int(u32),
    String(String),
}

#[derive(Deserialize, Debug)]
struct Day {
    date: Date,
    content: String,
    suggestion: String,
}

#[derive(Clone, Data, Lens)]
struct State {
    day: u32,
    #[data(ignore)]
    dispatch: Sender<u32>,
}

// #[tokio::main]
fn main() -> Result<(), PlatformError> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        while let res = rx.recv() {
            match res {
                Ok(day) => {
                    println!("{}", day);
                    thread::spawn(move || {
                        let res = get_data(day).unwrap();
                        println!("{:?}", res);
                    });
                },
                Err(err) => eprintln!("{:?}", err),
            }
        }
    });
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(State {
            day: data,
            dispatch: tx.clone()
        })

}

fn ui_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.day.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut State, _env| {
            data.day += 1;
            data.dispatch.send(data.day);
        })
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}

fn get_data(day: u32) -> Result<Day, Error> {
    let url = format!("https://day.ebichu.cc/api/{}", day);
    let response = reqwest::blocking::get(&url)?;
    let day: Day = response.json()?;
    Ok(day)
}
