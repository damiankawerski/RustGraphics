use crate::app::lab2::lab::utils::{lab_to_xyz, rgb_to_xyz, xyz_to_lab, xyz_to_rgb};

pub fn modify_lab(image: &mut image::RgbImage, l: i16, a: i16, b: i16) {
    let l_delta = l as f32 * (100.0 / 255.0);
    let a_delta = a as f32 * (128.0 / 255.0);
    let b_delta = b as f32 * (128.0 / 255.0);

    for pixel in image.pixels_mut() {
        let (x, y, z) = rgb_to_xyz(pixel[0], pixel[1], pixel[2]);

        let (l_lab, a_lab, b_lab) = xyz_to_lab(x, y, z);

        let new_l = (l_lab + l_delta).clamp(0.0, 100.0);
        let new_a = (a_lab + a_delta).clamp(-128.0, 127.0);
        let new_b = (b_lab + b_delta).clamp(-128.0, 127.0);

        let (x_new, y_new, z_new) = lab_to_xyz(new_l, new_a, new_b);

        let (r, g, blue) = xyz_to_rgb(x_new, y_new, z_new);
        *pixel = image::Rgb([r, g, blue]);
    }
}
