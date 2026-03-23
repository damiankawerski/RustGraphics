use super::image_manipulation::{change_brightness, change_contrast, change_gamma};

pub struct RGBControls {
    pub brightness: i16,
    pub contrast: i16,
    pub gamma: i16,
    is_modal_open: bool,
}

impl Default for RGBControls {
    fn default() -> Self {
        Self {
            brightness: 1,
            contrast: 1,
            gamma: 1,
            is_modal_open: false,
        }
    }
}

impl RGBControls {
    pub fn reset(&mut self) {
        self.brightness = 1;
        self.contrast = 1;
        self.gamma = 1;
    }

    pub fn draw_open_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("RGB").clicked() {
            self.is_modal_open = true;
        }
    }

    pub fn show_modal(&mut self, ctx: &egui::Context) -> bool {
        let mut changed = false;
        let mut is_modal_open = self.is_modal_open;

        egui::Window::new("RGB")
            .open(&mut is_modal_open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                changed |= self.draw_sliders(ui);

                ui.add_space(8.0);
                if ui.button("Resetuj").clicked() {
                    self.reset();
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
        adjusted = change_brightness(&adjusted, self.brightness);
        adjusted = change_contrast(&adjusted, self.contrast);
        adjusted = change_gamma(&adjusted, self.gamma);

        let (w, h) = adjusted.dimensions();
        let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], adjusted.as_raw());
        let texture =
            ctx.load_texture("processed", color_image, egui::TextureOptions::default());

        Some((adjusted, texture))
    }

    fn draw_sliders(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.brightness, -255..=255)
                    .text("Jasność")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.contrast, -255..=255)
                    .text("Kontrast")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.gamma, -255..=255)
                    .text("Gamma")
                    .integer(),
            )
            .changed();

        changed
    }
}