use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn bronya_holdsign(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("bronya_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(190, 675, 640, 930),
        text,
        25.0,
        60.0,
        text_params!(paint = new_paint(Color::from_rgb(111, 95, 95))),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "bronya_holdsign",
    bronya_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["V我50"],
    keywords = &["布洛妮娅举牌", "大鸭鸭举牌"],
    tags = MemeTags::bronya(),
    date_created = local_date(2022, 10, 27),
    date_modified = local_date(2023, 3, 30),
);
