use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Date {
    Int(u32),
    String(String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Day {
    date: Date,
    content: String,
    suggestion: String,
}
