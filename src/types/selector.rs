use druid::{Selector};

pub const DAY_DATA: Selector<String> = Selector::new("day-data");
pub const MENU_COUNT_ACTION: Selector<usize> = Selector::new("menu-count-action");
pub const CONCURRENCY_COUNT: Selector<u32> = Selector::new("concurrency-count");
pub const DUMMY_WINDOW: Selector<()> = Selector::new("dummy-window");
