use crate::types::day::{Day};
use reqwest::Error;

pub fn hello() {
  println!("xxx");
}

pub async fn get_data(day: u32) -> Result<Day, Error> {
    let url = format!("https://day.ebichu.cc/api/{}", day);
    let response = reqwest::get(&url).await?;
    let day: Day = response.json().await?;
    Ok(day)
}
