
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background: [u8; 3]) -> Self {
        let mut pixels = vec![0; (width * height * 3) as usize];
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.copy_from_slice(&background);
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn clear(&mut self, color: [u8; 3]) {
        for chunk in self.pixels.chunks_exact_mut(3) {
            chunk.copy_from_slice(&color);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        if x < 0 || y < 0 {
            return;
        }

        let (x, y) = (x as u32, y as u32);
        if x >= self.width || y >= self.height {
            return;
        }

        let index = ((y * self.width + x) * 3) as usize;
        self.pixels[index..index + 3].copy_from_slice(&color);
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: [u8; 3]) {
        // Brassenham algorytm do rysowania linii
        let mut x = x0;
        let mut y = y0;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.set_pixel(x, y, color);

            if x == x1 && y == y1 {
                break;
            }

            let e2 = 2 * err;

            if e2 >= dy {
                err += dy;
                x += sx;
            }

            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: [u8; 3]) {
        // Brassenham algorytm do rysowania ale od środka symetrii zamaist ze stałymi 
        // Z jakiegoś powodu brassenham z wykładu nie działa dobrze.
        if radius < 0 {
            return;
        }

        let mut x = radius;
        let mut y = 0;
        let mut decision = 1 - radius;

        while x >= y {
            self.plot_circle_points(cx, cy, x, y, color);
            y += 1;

            if decision < 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }

    }

    fn plot_circle_points(&mut self, cx: i32, cy: i32, x: i32, y: i32, color: [u8; 3]) {
        // Symetria wszystkich kombinacji wokół środka .
        self.set_pixel(cx + x, cy + y, color);
        self.set_pixel(cx + y, cy + x, color);
        self.set_pixel(cx - y, cy + x, color);
        self.set_pixel(cx - x, cy + y, color);
        self.set_pixel(cx - x, cy - y, color);
        self.set_pixel(cx - y, cy - x, color);
        self.set_pixel(cx + y, cy - x, color);
        self.set_pixel(cx + x, cy - y, color);
    }

    pub fn to_color_image(&self) -> egui::ColorImage {
        egui::ColorImage::from_rgb([self.width as usize, self.height as usize], &self.pixels)
    }
}