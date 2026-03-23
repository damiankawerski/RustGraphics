use crate::app::lab2::hsl::utils::{hsl_to_rgb, rgb_to_hsl};

pub fn modify_hsl(image: &mut image::RgbImage, h: i16, s: i16, l: i16) {
    for pixel in image.pixels_mut() {
        let (hue, saturation, lightness) = rgb_to_hsl(pixel[0], pixel[1], pixel[2]);

        let new_hue = (hue + h as f32).rem_euclid(360.0);
        let new_saturation = (saturation + (s as f32 / 255.0)).clamp(0.0, 1.0);
        let new_lightness = (lightness + (l as f32 / 255.0)).clamp(0.0, 1.0);

        let (r, g, b) = hsl_to_rgb(new_hue, new_saturation, new_lightness);
        *pixel = image::Rgb([r, g, b]);
    }
}
