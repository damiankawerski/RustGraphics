use crate::app::lab2::lab::image_manipulation::modify_lab;

pub struct LABControls {
    pub L: i16,
    pub a: i16,
    pub b: i16,
    is_modal_open: bool,
}

impl Default for LABControls {
    fn default() -> Self {
        Self {
            L: 0,
            a: 0,
            b: 0,
            is_modal_open: false,
        }
    }
}

impl LABControls {
    pub fn reset(&mut self) {
        self.L = 0;
        self.a = 0;
        self.b = 0;
    }

    pub fn draw_open_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("LAB").clicked() {
            self.is_modal_open = true;
        }
    }

    pub fn show_modal(&mut self, ctx: &egui::Context) -> bool {
        let mut changed = false;
        let mut is_modal_open = self.is_modal_open;

        egui::Window::new("LAB")
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

        if self.L != 0 || self.a != 0 || self.b != 0 {
            modify_lab(&mut adjusted, self.L, self.a, self.b);
        }

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
                egui::Slider::new(&mut self.L, -255..=255)
                    .text("L")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.a, -255..=255)
                    .text("a")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.b, -255..=255)
                    .text("b")
                    .integer(),
            )
            .changed();

        changed
    }
}