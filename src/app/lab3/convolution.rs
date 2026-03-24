use image::{Rgb, RgbImage};

fn conv(image: &RgbImage, mask: &[Vec<i16>], x: i32, y: i32, mut weight_sum: i32) -> (i32, i32, i32, i32) {
    let mask_size = mask.len() as i32;
    let offset = mask_size / 2;
    
    let mut r_sum: i32 = 0;
    let mut g_sum: i32 = 0;
    let mut b_sum: i32 = 0;

    for j in 0..mask_size {
        for i in 0..mask_size {
            let mi = (mask_size - 1 - i) as usize;
            let mj = (mask_size - 1 - j) as usize;

            let weight = mask[mj][mi] as i32;

            let px = (x + i - offset).clamp(0, image.width() as i32 - 1) as u32;
            let py = (y + j - offset).clamp(0, image.height() as i32 - 1) as u32;

            let pixel = image.get_pixel(px, py);

            r_sum += pixel[0] as i32 * weight;
            g_sum += pixel[1] as i32 * weight;
            b_sum += pixel[2] as i32 * weight;

            weight_sum += weight;
        }
    }

    return (r_sum, g_sum, b_sum, weight_sum);
}

pub fn apply_filter(image: &RgbImage, mask: &[Vec<i16>]) -> RgbImage {
    let mut output = RgbImage::new(image.width(), image.height());

    for y in 0..image.height() as i32 {
        for x in 0..image.width() as i32 {
            let weight_sum = 0; 

            let (mut r_sum, mut g_sum,mut b_sum, weight_sum) = conv(image, mask, x, y, weight_sum);

            let weight_sum = if weight_sum == 0 { 1 } else { weight_sum };

            r_sum /= weight_sum;
            g_sum /= weight_sum;
            b_sum /= weight_sum;

            output.put_pixel(
                x as u32,
                y as u32,
                Rgb([
                    r_sum.clamp(0, 255) as u8,
                    g_sum.clamp(0, 255) as u8,
                    b_sum.clamp(0, 255) as u8,
                ]),
            );
        }
    }

    output
}
