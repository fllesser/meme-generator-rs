use skia_safe::{AlphaType, Codec, ColorType, Image, ImageInfo, codec};

use meme_generator_core::error::Error;

pub trait CodecExt {
    fn is_multi_frame(&mut self) -> bool;

    fn get_average_duration(&mut self) -> Result<f32, Error>;

    fn first_frame(&mut self) -> Result<Image, Error>;

    fn get_frame(&mut self, index: usize) -> Result<Image, Error>;
}

impl<'a> CodecExt for Codec<'a> {
    fn is_multi_frame(&mut self) -> bool {
        self.get_frame_count() > 1
    }

    fn get_average_duration(&mut self) -> Result<f32, Error> {
        let count = self.get_frame_count();
        let mut total_duration = 0.0;
        for i in 0..count {
            let frame_info = self
                .get_frame_info(i)
                .ok_or(Error::ImageDecodeError("Skia decode error".to_string()))?;
            total_duration += frame_info.duration as f32 / 1000.0;
        }
        Ok(total_duration / count as f32)
    }

    fn first_frame(&mut self) -> Result<Image, Error> {
        self.get_frame(0)
    }

    fn get_frame(&mut self, index: usize) -> Result<Image, Error> {
        let image_info = ImageInfo::new(
            self.dimensions(),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let options = codec::Options {
            zero_initialized: codec::ZeroInitialized::No,
            subset: None,
            frame_index: index,
            prior_frame: None,
        };
        self.get_image(image_info, &options)
            .map_err(|err| Error::ImageDecodeError(format!("Skia decode error: {err:?}")))
    }
}
