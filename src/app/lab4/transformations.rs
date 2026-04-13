use egui::DragValue;
use image::RgbImage;
use super::utils::{resize, rotate};

pub struct Transformations {
    t_width: u32,
    t_height: u32,
    rotation_angle: i32,
    is_modal_open: bool,
}

impl Default for Transformations {
    fn default() -> Self {
        Self {
            t_width: 100,
            t_height: 100,
            rotation_angle: 0,
            is_modal_open: false,
        }
    }
}

impl Transformations {
    pub fn reset(
        &mut self,
        ctx: &egui::Context,
        original_image: &Option<RgbImage>,
        processed_image: &mut Option<RgbImage>,
        texture: &mut Option<egui::TextureHandle>,
    ) {
        if let Some(orig) = original_image {
            let img = orig.clone();
            let (w, h) = img.dimensions();
            let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], img.as_raw());

            self.t_width = w;
            self.t_height = h;
            self.rotation_angle = 0;

            let tex = ctx.load_texture("processed", color_image, egui::TextureOptions::default());
            *processed_image = Some(img);
            *texture = Some(tex);
        }
    }

    pub fn draw_open_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Transformacje").clicked() {
            self.is_modal_open = true;
        }
    }

    pub fn show_modal(
        &mut self,
        ctx: &egui::Context,
        original_image: &Option<RgbImage>,
        processed_image: &mut Option<RgbImage>,
        texture: &mut Option<egui::TextureHandle>,
    ) -> bool {
        let mut changed = false;
        let mut is_modal_open = self.is_modal_open;

        egui::Window::new("Transformacje")
            .open(&mut is_modal_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Szerokość:");
                    ui.add(
                        DragValue::new(&mut self.t_width)
                            .range(1..=10_000)
                            .speed(1),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Wysokość:");
                    ui.add(
                        DragValue::new(&mut self.t_height)
                            .range(1..=10_000)
                            .speed(1),
                    );
                });

                ui.add(
                    egui::Slider::new(&mut self.rotation_angle, -360..=360)
                        .text("Kąt obrotu"),
                );

                ui.add_space(8.0);

                if ui.button("Aplikuj").clicked() {
                    if let Some((img, tex)) = self.apply_all_processing(ctx, original_image) {
                        *processed_image = Some(img);
                        *texture = Some(tex);
                    }
                    changed = true;
                }

                if ui.button("Resetuj").clicked() {
                    self.reset(ctx, original_image, processed_image, texture);
                    changed = true;
                }
            });

        self.is_modal_open = is_modal_open;
        changed
    }

    pub fn apply_all_processing(
        &self,
        ctx: &egui::Context,
        original_image: &Option<RgbImage>,
    ) -> Option<(RgbImage, egui::TextureHandle)> {
        let original = original_image.as_ref()?;

        let resized = resize(original, self.t_width, self.t_height);

        let rotated = rotate(&resized, self.rotation_angle as f32);

        let (w, h) = rotated.dimensions();
        let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], rotated.as_raw());
        let texture = ctx.load_texture("processed", color_image, egui::TextureOptions::default());

        Some((rotated, texture))
    }
}

