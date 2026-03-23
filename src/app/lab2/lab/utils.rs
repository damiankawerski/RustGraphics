fn norm_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f32) -> f32 {
    let c = c.max(0.0);
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

pub fn rgb_to_xyz(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let (rf, gf, bf) = norm_rgb(r, g, b);
    let rl = srgb_to_linear(rf);
    let gl = srgb_to_linear(gf);
    let bl = srgb_to_linear(bf);

    let x = 0.4124564 * rl + 0.3575761 * gl + 0.1804375 * bl;
    let y = 0.2126729 * rl + 0.7151522 * gl + 0.0721750 * bl;
    let z = 0.0193339 * rl + 0.1191920 * gl + 0.9503041 * bl;
    (x, y, z)
}

pub fn xyz_to_rgb(x: f32, y: f32, z: f32) -> (u8, u8, u8) {
    let rl = 3.2404542 * x - 1.5371385 * y - 0.4985314 * z;
    let gl = -0.9692660 * x + 1.8760108 * y + 0.0415560 * z;
    let bl = 0.0556434 * x - 0.2040259 * y + 1.0572252 * z;

    let r = (linear_to_srgb(rl).clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (linear_to_srgb(gl).clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (linear_to_srgb(bl).clamp(0.0, 1.0) * 255.0).round() as u8;

    (r, g, b)
}

pub fn xyz_to_lab(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let x_r = 0.95047;
    let y_r = 1.0;
    let z_r = 1.08883;

    let xr = x / x_r;
    let yr = y / y_r;
    let zr = z / z_r;

    let fx_v = fx(xr);
    let fy_v = fx(yr);
    let fz_v = fx(zr);

    (116.0 * fy_v - 16.0, 500.0 * (fx_v - fy_v), 200.0 * (fy_v - fz_v))
}

pub fn lab_to_xyz(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    const EPSILON: f32 = 0.008856;
    const KAPPA: f32 = 903.3;

    let x_r = 0.95047;
    let y_r = 1.0;
    let z_r = 1.08883;

    let fy = (l + 16.0) / 116.0;
    let yr = if l > KAPPA * EPSILON {
        fy.powi(3)
    } else {
        l / KAPPA
    };

    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;

    let fx3 = fx.powi(3);
    let fz3 = fz.powi(3);

    let xr = if fx3 > EPSILON {
        fx3
    } else {
        (116.0 * fx - 16.0) / KAPPA
    };

    let zr = if fz3 > EPSILON {
        fz3
    } else {
        (116.0 * fz - 16.0) / KAPPA
    };

    (xr * x_r, yr * y_r, zr * z_r)
}


fn fx(x: f32) -> f32 {
    if x > 0.008856 {
        x.powf(1.0 / 3.0)
    } else {
        (x * 903.3 + 16.0) / 116.0
    }
}