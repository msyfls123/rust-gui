use druid::{
  MenuDesc, MenuItem, LocalizedString, Command, Target,
  SysMods,
};

use crate::types::{day, selector, State};
use selector::{MENU_COUNT_ACTION};

pub fn make_demo_menu() -> MenuDesc<State> {
    let submenu = MenuDesc::new(LocalizedString::new("demo-menu")).append_iter(|| (0..4).map(|i| {
        MenuItem::new(
            LocalizedString::new("demo-menu-item").with_arg("count", move |_, _| i.into()),
            Command::new(MENU_COUNT_ACTION, i, Target::Auto),
        ).hotkey(SysMods::Cmd, &*i.to_string())
    }));
    submenu
}
