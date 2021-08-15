use druid::{
  Data,
  Lens,
};
use druid::im::{Vector};
use druid::text::{RichText};

use tokio::sync::mpsc::{UnboundedSender};

pub mod day;
pub mod selector;
pub mod rich_editor;

#[derive(Debug, Clone, Data, Lens)]
pub struct State {
    pub day: u32,
    pub concurrency: u32,
    #[data(ignore)]
    pub dispatch: UnboundedSender<u32>,
    pub day_data: String,
    pub days: Vector<String>,
    pub color_index: usize,
    pub rich_raw: String,
    pub rich_text: RichText,
    pub recent_open_win_time: u128,
}
