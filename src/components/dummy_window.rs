use druid::{Widget, Color, WidgetExt};
use druid::widget::{
  Flex, Label,
  FlexParams, CrossAxisAlignment, MainAxisAlignment,
};

use crate::types::{State};

pub fn dummy_window_builder() -> impl Widget<State> {
  Flex::row()
    .main_axis_alignment(MainAxisAlignment::SpaceAround)
    .must_fill_main_axis(true)
    .with_flex_child(Label::new("dummy")
      .expand_height()
      .padding(20.0)
      .border(Color::RED, 2.0),
      FlexParams::new(2.0, CrossAxisAlignment::End)
    )
    .with_child(Label::new("code")
      .padding(20.0)
      .border(Color::GREEN, 2.0)
    )
}
