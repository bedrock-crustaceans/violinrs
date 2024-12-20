pub mod blend_modes;

use std::cmp::max;
use std::io::stderr;
use std::ops::Index;
use hsl::HSL;
use image::{GenericImage, ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use std::path::PathBuf;
use crate::image::blend_modes::{overlay_blend_mode, BlendMode};
// use photon_rs::PhotonImage;

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

    pub fn compose(&self, other: Self, blend_mode: BlendMode) -> Image {
        let new_src = match blend_mode {
            BlendMode::Overlay => {
                compose_overlay(self.clone(), other)
            }
        };

        Self {
            source: self.source.clone(),
            img: new_src,
            hue_shift: self.hue_shift,
        }
    }
}

// fn photon_from_rgba_image(img: RgbaImage) -> PhotonImage {
//     PhotonImage::new(img.as_raw().clone(), img.width(), img.height())
// }
// 
// fn rgba_image_from_photon(img: PhotonImage) -> RgbaImage {
//     RgbaImage::from_raw(img.get_width(), img.get_height(), img.get_raw_pixels()).unwrap()
// }
// 
// fn compose(a: RgbaImage, b: RgbaImage, blend_mode: &str) -> RgbaImage {
//     let mut ai = photon_from_rgba_image(a);
//     let mut bi = photon_from_rgba_image(b);
// 
//     photon_rs::multiple::blend(&mut ai, &bi, blend_mode);
// 
//     rgba_image_from_photon(ai)
// }

fn compose_overlay(a: Image, b: Image) -> RgbaImage {
    let a_src = a.img.clone();
    let b_src = b.img.clone();

    let mut result_src = RgbaImage::new(a_src.width(), a_src.height());

    for (x, y, color) in result_src.enumerate_pixels_mut() {
        let a_channels: Vec<f64> = a_src.get_pixel(x, y).clone().channels().iter().map(|x| x.clone() as f64 / 255.0).collect();
        let b_channels: Vec<f64> = b_src.get_pixel(x, y).clone().channels().iter().map(|x| x.clone() as f64 / 255.0).collect();
        let mut ar = a_channels[0]; let mut ag = a_channels[1]; let mut ab = a_channels[2]; let aa = a_channels[3];
        let br = b_channels[0]; let bg = b_channels[1]; let bb = b_channels[2]; let ba = b_channels[3];
        // let [mut br, mut bg, mut bb, mut ba]: [&f64] = b_src.get_pixel(x, y).clone().channels().iter().map(|x| x.clone() as f64 / 255.0).collect() else {
        //     unreachable!()
        // };

        ar = overlay_blend_mode(ar, br, ba);
        ag = overlay_blend_mode(ag, bg, ba);
        ab = overlay_blend_mode(ab, bg, ba);

        *color = Rgba([(ar * 255.0) as u8, (ag * 255.0) as u8, (ab * 255.0) as u8, (aa * 255.0) as u8]);
    }

    result_src
}