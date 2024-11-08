use hsl::HSL;
use image::{GenericImage, ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Image {
    source: PathBuf,
    hue_shift: f64,
    img: RgbaImage,
}

impl Image {
    pub fn new(src: impl Into<PathBuf> + Clone) -> Self {
        Self {
            source: src.clone().into(),
            hue_shift: 0.0,
            img: image::ImageReader::open(src.into())
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8(),
        }
    }

    pub fn src(&self) -> PathBuf {
        self.source.clone()
    }

    pub fn build(&self, path: PathBuf) {
        &self.img.save(&path).unwrap();
    }

    pub fn with_hue_shift(&mut self, amount: f64) -> Self {
        self.hue_shift = amount;

        for (x, y, color) in self.img.enumerate_pixels_mut() {
            *color = Self::shifted(*color, amount)
        }

        self.clone()
    }

    fn shifted(color: Rgba<u8>, amount: f64) -> Rgba<u8> {
        let channels = color.channels();
        let fr = channels[0];
        let fg = channels[1];
        let fb = channels[2];
        let fa = channels[3];
        let mut hsl = HSL::from_rgb(&[fr, fg, fb]);
        hsl.h += amount;
        let (r, g, b) = hsl.to_rgb();
        Rgba::from([r, g, b, fa.clone()])
    }

    pub fn upscaled(&self, amount: u32) -> Self {
        let mut upscaled = self.clone();

        let mut img = upscaled.img;

        let mut buf = RgbaImage::new(
            img.width() * amount,
            img.height() * amount,
        );

        for (ox, oy, color) in img.enumerate_pixels() {
            for y in 0..amount {
                for x in 0..amount {
                    buf.put_pixel(
                        ox * amount + x,
                        oy * amount + y,
                        color.clone()
                    );
                }
            }
        }

        Self {
            source: self.source.clone(),
            img: buf,
            hue_shift: self.hue_shift,
        }
    }
}
