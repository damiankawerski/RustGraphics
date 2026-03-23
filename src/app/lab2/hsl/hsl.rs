use super::image_manipulation::modify_hsl;

pub struct HSLControls {
    pub h: i16,
    pub s: i16,
    pub l: i16,
    is_modal_open: bool,
}

impl Default for HSLControls {
    fn default() -> Self {
        Self {
            h: 0,
            s: 0,
            l: 0,
            is_modal_open: false,
        }
    }
}

impl HSLControls {
    pub fn reset(&mut self) {
        self.h = 0;
        self.s = 0;
        self.l = 0;
    }

    pub fn draw_open_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("HSL").clicked() {
            self.is_modal_open = true;
        }
    }

    pub fn show_modal(&mut self, ctx: &egui::Context) -> bool {
        let mut changed = false;
        let mut is_modal_open = self.is_modal_open;

        egui::Window::new("HSL")
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

        modify_hsl(&mut adjusted, self.h, self.s, self.l);

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
                egui::Slider::new(&mut self.h, -180..=180)
                    .text("H")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.s, -255..=255)
                    .text("S")
                    .integer(),
            )
            .changed();

        ui.add_space(8.0);

        ui.spacing_mut().slider_width = ui.available_width() - 120.0;
        changed |= ui
            .add(
                egui::Slider::new(&mut self.l, -255..=255)
                    .text("L")
                    .integer(),
            )
            .changed();

        changed
    }
}