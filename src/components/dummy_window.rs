use druid::{Widget, Color, WidgetExt};
use druid::widget::{
  Flex, Label,
  FlexParams, CrossAxisAlignment,
};

use crate::types::{State};

pub fn dummy_window_builder() -> impl Widget<State> {
  Flex::row()
    .with_flex_child(Label::new("dummy")
      .padding(20.0)
      .border(Color::RED, 2.0),
      FlexParams::new(0.5, CrossAxisAlignment::End)
    )
    .with_child(Label::new("code")
      .padding(20.0)
      .border(Color::GREEN, 2.0)
    )
}
