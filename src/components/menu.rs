use druid::{
  MenuDesc, MenuItem, LocalizedString, Command, Target,
};

use crate::types::{day, selector, State};
use selector::{MENU_COUNT_ACTION};

pub fn make_demo_menu() -> MenuDesc<State> {
    let submenu = MenuDesc::new(LocalizedString::new("hello2")).append_iter(|| (0..4).map(|i| {
        MenuItem::new(
            LocalizedString::new("hello-counter").with_arg("count", move |_, _| i.into()),
            Command::new(MENU_COUNT_ACTION, i, Target::Auto),
        )
    }));
    submenu
}
