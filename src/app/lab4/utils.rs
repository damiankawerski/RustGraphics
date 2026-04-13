use image::{Rgb, RgbImage};

fn interpolate(x: f32, y: f32, img: &RgbImage) -> Rgb<u8> {
    let (w, h) = img.dimensions();
    let mut x1 = x.floor() as u32;
    let mut y1 = y.floor() as u32;
    let mut x2 = x1 + 1;
    let mut y2 = y1 + 1;

    // Odcinanie wartości poza granicami obrazu
    let dy = y - y1 as f32;
    let ddy = 1.0 - dy;
    let dx = x - x1 as f32;
    let ddx = 1.0 - dx;

    x1 = x1.clamp(0, w - 1);
    y1 = y1.clamp(0, h - 1);
    x2 = x2.clamp(0, w - 1);
    y2 = y2.clamp(0, h - 1);

    // Piksele

    let p11 = img.get_pixel(x1, y1);
    let p12 = img.get_pixel(x1, y2);
    let p21 = img.get_pixel(x2, y1);
    let p22 = img.get_pixel(x2, y2);

    // Interpolacja wzdłuż osi Y

    let r1 = dy * p12[0] as f32 + ddy * p11[0] as f32;
    let g1 = dy * p12[1] as f32 + ddy * p11[1] as f32;
    let b1 = dy * p12[2] as f32 + ddy * p11[2] as f32;

    let r2 = dy * p22[0] as f32 + ddy * p21[0] as f32;
    let g2 = dy * p22[1] as f32 + ddy * p21[1] as f32;
    let b2 = dy * p22[2] as f32 + ddy * p21[2] as f32;

    // Interpolacja wzdłuż osi X

    let r = (ddx * r1 + dx * r2).round() as u8;
    let g = (ddx * g1 + dx * g2).round() as u8;
    let b = (ddx * b1 + dx * b2).round() as u8;

    Rgb([r, g, b])
}

fn apply_interpolation(
    src: &RgbImage,
    dst_w: u32,
    dst_h: u32,
    row0: (f32, f32, f32),
    row1: (f32, f32, f32),
) -> RgbImage {
    let mut dst = RgbImage::from_pixel(dst_w, dst_h, Rgb([0, 0, 0]));

    let src_w = src.width() as f32;
    let src_h = src.height() as f32;

    for y_dst in 0..dst_h {
        for x_dst in 0..dst_w {
            let x_dst_f = x_dst as f32;
            let y_dst_f = y_dst as f32;

            let x_src = row0.0 * x_dst_f + row0.1 * y_dst_f + row0.2;
            let y_src = row1.0 * x_dst_f + row1.1 * y_dst_f + row1.2;

            if x_src < -0.5 || x_src >= src_w - 0.5 || y_src < -0.5 || y_src >= src_h - 0.5 {
                dst.put_pixel(x_dst, y_dst, Rgb([0, 0, 0]));
                continue;
            }

            let pixel = interpolate(x_src, y_src, src);

            dst.put_pixel(x_dst, y_dst, pixel);
        }
    }

    dst
}

pub fn resize(src: &RgbImage, dst_w: u32, dst_h: u32) -> RgbImage {
    let src_w = src.width() as f32;
    let src_h = src.height() as f32;

    let m0 = src_w / dst_w as f32;
    let m1 = src_h / dst_h as f32;

    apply_interpolation(src, dst_w, dst_h, (m0, 0.0, 0.0), (0.0, m1, 0.0))
}

pub fn rotate(src: &RgbImage, angle_deg: f32) -> RgbImage {
    let angle = angle_deg.to_radians();
    let cos_angle = angle.cos();
    let sin_angle = angle.sin();

    let src_w = src.width() as f32;
    let src_h = src.height() as f32;

    // Środek obrazu 
    let cx_src = src_w / 2.0;
    let cy_src = src_h / 2.0;

    // bounding box po obrocie czterech narożników
    let corners_x = [0.0, src_w, src_w, 0.0];
    let corners_y = [0.0, 0.0, src_h, src_h];

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for i in 0..4 {
        let dx = corners_x[i] - cx_src;
        let dy = corners_y[i] - cy_src;
        let rx = cos_angle * dx - sin_angle * dy;
        let ry = sin_angle * dx + cos_angle * dy;

        min_x = min_x.min(rx);
        max_x = max_x.max(rx);
        min_y = min_y.min(ry);
        max_y = max_y.max(ry);
    }

    let dst_w = (max_x - min_x).ceil() as u32;
    let dst_h = (max_y - min_y).ceil() as u32;

    // środek obrazu docelowego
    let cx_dst = dst_w as f32 / 2.0;
    let cy_dst = dst_h as f32 / 2.0;

    // macierz odwrotna rotacji 
    let tx = cx_src - cos_angle * cx_dst - sin_angle * cy_dst;
    let ty = cy_src + sin_angle * cx_dst - cos_angle * cy_dst;

    let dst = apply_interpolation(
        src,
        dst_w,
        dst_h,
        (cos_angle, sin_angle, tx),
        (-sin_angle, cos_angle, ty),
    );

    dst
}
