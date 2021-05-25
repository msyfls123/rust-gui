use druid::widget::{Image, SizedBox};
use druid::{Widget, ImageBuf, WidgetExt, Color};
use druid::piet::{ImageFormat};

use crate::types::State;

pub fn make_girl() -> impl Widget<State> {
  let raw_image = include_bytes!("../../resources/image/girl.jpg");
  let image_data = image::load_from_memory(raw_image).map_err(|e| e).unwrap();
  let rgb_image = image_data.to_rgb8();
  let sizeofimage = rgb_image.dimensions();
  let girl_data = ImageBuf::from_raw(
    rgb_image.to_vec(),
    ImageFormat::Rgb,
    sizeofimage.0 as usize,
    sizeofimage.1 as usize,
  );
  SizedBox::new(Image::new(girl_data))
    .fix_width(sizeofimage.0 as f64 / 8.0)
    .fix_height(sizeofimage.1 as f64 / 8.0)
    .border(Color::grey(0.6), 2.0).center().boxed()
}
