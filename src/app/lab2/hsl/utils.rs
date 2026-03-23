pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    // Normalize
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;

    // Lightness
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let dm = max - min;
    let l = (max + min) / 2.0;

    // Saturation
    let s = if dm == 0.0 {
        0.0
    } else if l <= 0.5 {
        dm / (2.0 * l)
    } else {
        dm / (2.0 - 2.0 * l)
    };

    // Hue
    let h = if dm == 0.0 {
        0.0
    } else if max == rf && gf > bf {
        60.0 * ((gf - bf) / dm)
    } else if max == rf && gf < bf {
        60.0 * ((gf - bf) / dm) + 360.0
    } else if max == gf {
        60.0 * ((bf - rf) / dm) + 120.0
    } else {
        60.0 * ((rf - gf) / dm) + 240.0
    };

    (h, s, l)
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    if s == 0.0 {
        return (
            (l * 255.0).round() as u8,
            (l * 255.0).round() as u8,
            (l * 255.0).round() as u8,
        );
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };

    let p = 2.0 * l - q;

    let h = h / 360.0;

    let tr = h + 1.0 / 3.0;
    let tg = h;
    let tb = h - 1.0 / 3.0;

    fn channel_from_tc(mut tc: f32, p: f32, q: f32) -> f32 {
        if tc < 0.0 {
            tc += 1.0;
        }
        if tc > 1.0 {
            tc -= 1.0;
        }

        if tc < 1.0 / 6.0 {
            p + (q - p) * 6.0 * tc
        } else if tc < 1.0 / 2.0 {
            q
        } else if tc < 2.0 / 3.0 {
            p + (q - p) * 6.0 * (2.0 / 3.0 - tc)
        } else {
            p
        }
    }

    let r = (channel_from_tc(tr, p, q) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = (channel_from_tc(tg, p, q) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = (channel_from_tc(tb, p, q) * 255.0).round().clamp(0.0, 255.0) as u8;

    (r, g, b)
}
