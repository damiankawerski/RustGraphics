pub fn calculate_histogram(image: &image::RgbImage) -> ([u32; 256], [u32; 256], [u32; 256]) {
    let mut red = [0u32; 256];
    let mut green = [0u32; 256];
    let mut blue = [0u32; 256];

    for pixel in image.pixels() {
        red[pixel[0] as usize] += 1;
        green[pixel[1] as usize] += 1;
        blue[pixel[2] as usize] += 1;
    }

    (red, green, blue)
}

