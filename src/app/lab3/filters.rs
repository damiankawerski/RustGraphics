use crate::app::lab3::convolution::apply_filter;
use egui::{DragValue, Grid};

pub struct Filters {
    mask: Vec<Vec<i16>>,
    size: usize,
    is_modal_open: bool,
}

impl Filters {
    fn create_ones_mask(size: usize) -> Vec<Vec<i16>> {
        let normalized_size = size.max(1);
        vec![vec![1_i16; normalized_size]; normalized_size]
    }
}

impl Default for Filters {
    fn default() -> Self {
        let size = 10;

        Self {
            mask: Self::create_ones_mask(size),
            size,
            is_modal_open: false,
        }
    }
}

impl Filters {
    pub fn reset(
        &mut self,
        ctx: &egui::Context,
        original_image: &Option<image::RgbImage>,
        processed_image: &mut Option<image::RgbImage>,
        texture: &mut Option<egui::TextureHandle>,
    ) {
        if let Some(orig) = original_image {
            let img = orig.clone();

            let (w, h) = img.dimensions();
            let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], img.as_raw());

            let tex = ctx.load_texture("processed", color_image, egui::TextureOptions::default());

            *processed_image = Some(img);
            *texture = Some(tex);
        }
    }

    pub fn draw_open_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Filtry").clicked() {
            self.is_modal_open = true;
        }
    }

    pub fn show_modal(
        &mut self,
        ctx: &egui::Context,
        original_image: &Option<image::RgbImage>,
        processed_image: &mut Option<image::RgbImage>,
        texture: &mut Option<egui::TextureHandle>,
    ) -> bool {
        let mut changed = false;
        let mut is_modal_open = self.is_modal_open;

        egui::Window::new("Filtry")
            .open(&mut is_modal_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                changed |= self.draw_matrix(ui);

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
        original_image: &Option<image::RgbImage>,
    ) -> Option<(image::RgbImage, egui::TextureHandle)> {
        let original = original_image.as_ref()?;

        let mut adjusted = original.clone();

        adjusted = apply_filter(&adjusted, &self.mask);

        let (w, h) = adjusted.dimensions();
        let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], adjusted.as_raw());
        let texture = ctx.load_texture("processed", color_image, egui::TextureOptions::default());

        Some((adjusted, texture))
    }

    fn draw_matrix(
        &mut self,
        ui: &mut egui::Ui,
    ) -> bool {
        ui.spacing_mut().slider_width = ui.available_width() - 120.0;

        let size_changed = ui
            .add(
                egui::Slider::new(&mut self.size, 1..=25)
                    .text("Size")
                    .integer(),
            )
            .changed();

        if size_changed {
            self.mask = Self::create_ones_mask(self.size);
        }

        Grid::new("matrix_grid").show(ui, |ui| {
            for row in &mut self.mask {
                for val in row {
                    ui.add(DragValue::new(val));
                }
                ui.end_row();
            }
        });

        size_changed
    }
}
