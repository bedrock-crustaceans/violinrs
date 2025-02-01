pub mod blend_modes;

use crate::image::blend_modes::{overlay_blend_mode, BlendMode};
use hsl::HSL;
use image::{Pixel, Rgba, RgbaImage};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Image {
    source: PathBuf,
    hue_shift: f64,
    img: RgbaImage,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            source: PathBuf::from(""),
            hue_shift: 0.0,
            img: RgbaImage::new(16, 16),
        }
    }
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
        let _ = &self.img.save(&path).unwrap();
    }

    pub fn with_hue_shift(&mut self, amount: f64) -> Self {
        self.hue_shift = amount;

        for (_, _, color) in self.img.enumerate_pixels_mut() {
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
        let upscaled = self.clone();

        let img = upscaled.img;

        let mut buf = RgbaImage::new(img.width() * amount, img.height() * amount);

        for (ox, oy, color) in img.enumerate_pixels() {
            for y in 0..amount {
                for x in 0..amount {
                    buf.put_pixel(ox * amount + x, oy * amount + y, color.clone());
                }
            }
        }

        Self {
            source: self.source.clone(),
            img: buf,
            hue_shift: self.hue_shift,
        }
    }

    pub fn compose(&self, other: Self, blend_mode: BlendMode, options: ComposeOptions) -> Image {
        let mut new_src = self.clone().img;

        for _ in 0..options.repeats {
            match blend_mode {
                BlendMode::Overlay => new_src = compose_overlay(new_src, other.img.clone()),
            };
        }

        Self {
            source: self.source.clone(),
            img: new_src,
            hue_shift: self.hue_shift,
        }
    }
}

fn compose_overlay(a: RgbaImage, b: RgbaImage) -> RgbaImage {
    let a_src = a;
    let b_src = b;

    let mut result_src = RgbaImage::new(a_src.width(), a_src.height());

    for (x, y, color) in result_src.enumerate_pixels_mut() {
        let a_channels: Vec<f64> = a_src
            .get_pixel(x, y)
            .clone()
            .channels()
            .iter()
            .map(|x| x.clone() as f64 / 255.0)
            .collect();
        let b_channels: Vec<f64> = b_src
            .get_pixel(x, y)
            .clone()
            .channels()
            .iter()
            .map(|x| x.clone() as f64 / 255.0)
            .collect();
        let mut ar = a_channels[0];
        let mut ag = a_channels[1];
        let mut ab = a_channels[2];
        let aa = a_channels[3];
        let br = b_channels[0];
        let bg = b_channels[1]; /*let bb = b_channels[2];*/
        let ba = b_channels[3];
        // let [mut br, mut bg, mut bb, mut ba]: [&f64] = b_src.get_pixel(x, y).clone().channels().iter().map(|x| x.clone() as f64 / 255.0).collect() else {
        //     unreachable!()
        // };

        ar = overlay_blend_mode(ar, br, ba);
        ag = overlay_blend_mode(ag, bg, ba);
        ab = overlay_blend_mode(ab, bg, ba);

        *color = Rgba([
            (ar * 255.0) as u8,
            (ag * 255.0) as u8,
            (ab * 255.0) as u8,
            (aa * 255.0) as u8,
        ]);
    }

    result_src
}

#[derive(Clone, Debug)]
pub struct ComposeOptions {
    pub repeats: u8,
}

impl Default for ComposeOptions {
    fn default() -> Self {
        Self { repeats: 1 }
    }
}

impl ComposeOptions {
    pub fn using_repeats(self, repeats: u8) -> Self {
        Self { repeats, ..self }
    }
}
