use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Date {
    Int(u32),
    String(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Day {
    day: Date,
    weekday: u64,
}
