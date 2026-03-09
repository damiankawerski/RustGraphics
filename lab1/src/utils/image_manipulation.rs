pub fn change_brightness(original: &image::RgbImage, factor: i16) -> image::RgbImage {
    let (w, h) = original.dimensions();
    image::ImageBuffer::from_fn(w, h, |x, y| {
        let p = original.get_pixel(x, y);
        image::Rgb([
            (p[0] as i16 + factor).clamp(0, 255) as u8,
            (p[1] as i16 + factor).clamp(0, 255) as u8,
            (p[2] as i16 + factor).clamp(0, 255) as u8,
        ])
    })
}

pub fn change_contrast(original: &image::RgbImage, factor: i16) -> image::RgbImage {
    let (w, h) = original.dimensions();
    image::ImageBuffer::from_fn(w, h, |x, y| {
        let p = original.get_pixel(x, y);
        image::Rgb([
            (((p[0] as i16 - 128) * factor + 128).clamp(0, 255)) as u8,
            (((p[1] as i16 - 128) * factor + 128).clamp(0, 255)) as u8,
            (((p[2] as i16 - 128) * factor + 128).clamp(0, 255)) as u8,
        ])
    })
}

pub fn change_gamma(original: &image::RgbImage, factor: i16) -> image::RgbImage {
    let gamma = if factor >= 0 {
        1.0 - (factor as f32 / 255.0) * 0.99  
    } else {
        1.0 + (-factor as f32 / 255.0) * 4.0  
    };

    let lookup: Vec<u8> = (0u16..=255)
        .map(|i| ((i as f32 / 255.0).powf(gamma) * 255.0).clamp(0.0, 255.0) as u8)
        .collect();

    let (w, h) = original.dimensions();
    image::ImageBuffer::from_fn(w, h, |x, y| {
        let p = original.get_pixel(x, y);
        image::Rgb([
            lookup[p[0] as usize],
            lookup[p[1] as usize],
            lookup[p[2] as usize],
        ])
    })
}
