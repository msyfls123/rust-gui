use druid::{
  Data,
  Lens,
};

use tokio::sync::mpsc::{UnboundedSender};

pub mod day;
pub mod selector;

#[derive(Debug, Clone, Data, Lens)]
pub struct State {
    pub day: u32,
    pub concurrency: u32,
    #[data(ignore)]
    pub dispatch: UnboundedSender<u32>,
    pub day_data: String,
    pub color_index: usize,
}
