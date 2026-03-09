use egui_plot::{Bar, BarChart, Plot};
use rust_picture_processor::utils::histogram::calculate_histogram;
use rust_picture_processor::utils::image_manipulation::{
    change_brightness, change_contrast, change_gamma,
};

pub struct App {
    original_image: Option<image::RgbImage>,
    processed_image: Option<image::RgbImage>,
    texture: Option<egui::TextureHandle>,
    brightness: i16,
    contrast: i16,
    gamma: i16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            original_image: None,
            processed_image: None,
            texture: None,
            brightness: 1,
            contrast: 1,
            gamma: 1,
        }
    }
}

impl App {
    fn load_image(&mut self, ctx: &egui::Context) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
            .pick_file()
        else {
            return;
        };

        let Ok(img) = image::open(&path) else {
            return;
        };

        let rgb = img.to_rgb8();
        let (w, h) = rgb.dimensions();
        let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], rgb.as_raw());

        self.texture =
            Some(ctx.load_texture("processed", color_image, egui::TextureOptions::default()));
        self.original_image = Some(rgb.clone());
        self.processed_image = Some(rgb);
        self.brightness = 1;
        self.contrast = 1;
        self.gamma = 1;
    }

    fn apply_processing(
        &mut self,
        ctx: &egui::Context,
        fun: fn(&image::RgbImage, i16) -> image::RgbImage,
        factor: i16,
    ) {
        let Some(original) = &self.original_image else {
            return;
        };
        let adjusted = fun(original, factor);
        let (w, h) = adjusted.dimensions();
        let color_image = egui::ColorImage::from_rgb([w as usize, h as usize], adjusted.as_raw());
        self.texture =
            Some(ctx.load_texture("processed", color_image, egui::TextureOptions::default()));
        self.processed_image = Some(adjusted);
    }

    fn paint_histogram(&self, ui: &mut egui::Ui) {
        let Some(image) = &self.processed_image else {
            return;
        };
        let (red, green, blue) = calculate_histogram(image);

        let make_bars = |hist: &[u32; 256], color: egui::Color32| -> Vec<Bar> {
            hist.iter()
                .enumerate()
                .map(|(i, &c)| Bar::new(i as f64, c as f64).fill(color))
                .collect()
        };

        Plot::new("histogram")
            .height(ui.available_height())
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(BarChart::new("red", make_bars(&red, egui::Color32::RED)));
                plot_ui.bar_chart(BarChart::new("green", make_bars(&green, egui::Color32::GREEN)));
                plot_ui.bar_chart(BarChart::new("blue", make_bars(&blue, egui::Color32::BLUE)));
            });
            
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);

            if ui.button("Otwórz zdjęcie").clicked() {
                self.load_image(ctx);
            }

            ui.spacing_mut().slider_width = ui.available_width() - 120.0;
            let brightness_changed = ui
                .add(
                    egui::Slider::new(&mut self.brightness, -255..=255)
                        .text("Jasność")
                        .integer(),
                )
                .changed();
            if brightness_changed {
                self.apply_processing(ctx, change_brightness, self.brightness);
            }

            ui.add_space(8.0);

            ui.spacing_mut().slider_width = ui.available_width() - 120.0;
            let contrast_changed = ui
                .add(
                    egui::Slider::new(&mut self.contrast, -255..=255)
                        .text("Kontrast")
                        .integer(),
                )
                .changed();
            if contrast_changed {
                self.apply_processing(ctx, change_contrast, self.contrast);
            }

            ui.add_space(8.0);

            ui.spacing_mut().slider_width = ui.available_width() - 120.0;
            let gamma_changed = ui
                .add(
                    egui::Slider::new(&mut self.gamma, -255..=255)
                        .text("Gamma")
                        .integer(),
                )
                .changed();
            if gamma_changed {
                self.apply_processing(ctx, change_gamma, self.gamma);
            }

            ui.add_space(8.0);

            ui.columns(2, |cols| {
                if let Some(texture) = &self.texture {
                    let available = cols[0].available_size();
                    let img_size = texture.size_vec2();
                    let scale = (available.x / img_size.x)
                        .min(available.y / img_size.y)
                        .min(1.0);
                    cols[0].image((texture.id(), img_size * scale));
                } else {
                    cols[0].label("Brak obrazu");
                }

                self.paint_histogram(&mut cols[1]);
            });
        });
    }
}
