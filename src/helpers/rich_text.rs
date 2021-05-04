use druid::{KeyOrValue, Color};
use druid::text::{Attribute, RichText};

pub fn get_initial_rich_text_data() -> RichText {
    let attr = Attribute::TextColor(KeyOrValue::Concrete(Color::PURPLE));
    RichText::new(String::from("hello world").into())
        .with_attribute(6..=10, attr)
}