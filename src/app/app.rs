use super::histogram::calculate_histogram;
use super::lab1::rgb::RGBControls;
use super::lab2::hsl::hsl::HSLControls;
use super::lab2::lab::lab::LABControls;
use egui_plot::{Bar, BarChart, Plot};

pub struct App {
    original_image: Option<image::RgbImage>,

    processed_image: Option<image::RgbImage>,

    texture: Option<egui::TextureHandle>,

    rgb_controls: RGBControls,

    hsl_controls: HSLControls,

    lab_controls: LABControls,
}

impl Default for App {
    fn default() -> Self {
        Self {
            original_image: None,
            processed_image: None,
            texture: None,
            rgb_controls: RGBControls::default(),
            hsl_controls: HSLControls::default(),
            lab_controls: LABControls::default(),
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
        self.rgb_controls.reset();
        self.hsl_controls.reset();
        self.lab_controls.reset();
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
                plot_ui.bar_chart(BarChart::new(
                    "green",
                    make_bars(&green, egui::Color32::GREEN),
                ));
                plot_ui.bar_chart(BarChart::new("blue", make_bars(&blue, egui::Color32::BLUE)));
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                if ui.button("Otwórz plik").clicked() {
                    self.load_image(ctx);
                }

                self.rgb_controls.draw_open_button(ui);
                self.hsl_controls.draw_open_button(ui);
                self.lab_controls.draw_open_button(ui);
            });

            if self.rgb_controls.show_modal(ctx) {
                if let Some((processed_image, texture)) = self
                    .rgb_controls
                    .apply_all_processing(ctx, &self.original_image)
                {
                    self.processed_image = Some(processed_image);
                    self.texture = Some(texture);
                }
            }

            if self.hsl_controls.show_modal(ctx) {
                if let Some((processed_image, texture)) = self
                    .hsl_controls
                    .apply_all_processing(ctx, &self.original_image)
                {
                    self.processed_image = Some(processed_image);
                    self.texture = Some(texture);
                }
            }

            if self.lab_controls.show_modal(ctx) {
                if let Some((processed_image, texture)) = self
                    .lab_controls
                    .apply_all_processing(ctx, &self.original_image)
                {
                    self.processed_image = Some(processed_image);
                    self.texture = Some(texture);
                }
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
