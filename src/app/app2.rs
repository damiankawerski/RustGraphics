use crate::app::lab5::canvas::Canvas;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DrawTool {
    Line,
    Circle,
}

pub struct App2 {
    canvas: Canvas,
    texture: Option<egui::TextureHandle>,
    active_tool: DrawTool,
    drag_start: Option<(i32, i32)>,
    drag_current: Option<(i32, i32)>,
}

impl Default for App2 {
    fn default() -> Self {
        let width = 1024;
        let height = 1024;

        Self {
            canvas: Canvas::new(width, height, [255, 255, 255]),
            texture: None,
            active_tool: DrawTool::Line,
            drag_start: None,
            drag_current: None,
        }
    }
}

impl App2 {
    fn refresh_texture(&mut self, ctx: &egui::Context) {
        let image = self.canvas.to_color_image();

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture = Some(ctx.load_texture("canvas", image, egui::TextureOptions::NEAREST));
        }
    }

    fn pointer_to_canvas(&self, image_rect: egui::Rect, pointer_pos: egui::Pos2) -> (i32, i32) {
        let x = ((pointer_pos.x - image_rect.left()) * self.canvas.width as f32 / image_rect.width())
            .floor() as i32;
        let y = ((pointer_pos.y - image_rect.top()) * self.canvas.height as f32 / image_rect.height())
            .floor() as i32;

        (
            x.clamp(0, self.canvas.width as i32 - 1),
            y.clamp(0, self.canvas.height as i32 - 1),
        )
    }

    fn canvas_to_screen(&self, image_rect: egui::Rect, point: (i32, i32)) -> egui::Pos2 {
        egui::pos2(
            image_rect.left() + (point.0 as f32 + 0.5) * image_rect.width() / self.canvas.width as f32,
            image_rect.top() + (point.1 as f32 + 0.5) * image_rect.height() / self.canvas.height as f32,
        )
    }

    fn draw_preview(&self, ui: &egui::Ui, image_rect: egui::Rect) {
        let (Some(start), Some(current)) = (self.drag_start, self.drag_current) else {
            return;
        };

        let painter = ui.painter();
        let stroke = egui::Stroke::new(0.3, egui::Color32::from_rgb(0, 0, 0));
        let start_screen = self.canvas_to_screen(image_rect, start);
        let current_screen = self.canvas_to_screen(image_rect, current);

        match self.active_tool {
            DrawTool::Line => {
                painter.line_segment([start_screen, current_screen], stroke);
            }
            DrawTool::Circle => {
                let dx = current.0 - start.0;
                let dy = current.1 - start.1;
                let radius_canvas = ((dx * dx + dy * dy) as f32).sqrt();
                let radius_screen = radius_canvas
                    * 0.5
                    * (image_rect.width() / self.canvas.width as f32
                        + image_rect.height() / self.canvas.height as f32);
                painter.circle_stroke(start_screen, radius_screen, stroke);
            }
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        if self.texture.is_none() {
            self.refresh_texture(&ctx);
        }

        ui.horizontal(|ui| {
            ui.label("Narzedzie:");
            ui.selectable_value(&mut self.active_tool, DrawTool::Line, "Linia");
            ui.selectable_value(&mut self.active_tool, DrawTool::Circle, "Okrag");

            if ui.button("Wyczysc canvas").clicked() {
                self.canvas.clear([255, 255, 255]);
                self.refresh_texture(&ctx);
            }
        });

        ui.separator();

        if let Some(texture) = &self.texture {
            let max_size = ui.available_size();
            let image_size = texture.size_vec2();
            let scale = (max_size.x / image_size.x)
                .min(max_size.y / image_size.y)
                .min(1.0);
            let response = ui.add(
                egui::Image::new((texture.id(), image_size * scale)).sense(egui::Sense::drag()),
            );

            if response.drag_started() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    let start = self.pointer_to_canvas(response.rect, pointer_pos);
                    self.drag_start = Some(start);
                    self.drag_current = Some(start);
                }
            }

            if response.dragged() {
                if let Some(pointer_pos) = response.interact_pointer_pos() {
                    self.drag_current = Some(self.pointer_to_canvas(response.rect, pointer_pos));
                }
            }

            if response.drag_stopped() {
                if let (Some(start), Some(end)) = (self.drag_start, self.drag_current) {
                    match self.active_tool {
                        DrawTool::Line => {
                            self.canvas.draw_line(start.0, start.1, end.0, end.1, [0, 0, 0]);
                        }
                        DrawTool::Circle => {
                            let dx = end.0 - start.0;
                            let dy = end.1 - start.1;
                            let radius = ((dx * dx + dy * dy) as f32).sqrt().round() as i32;
                            self.canvas.draw_circle(start.0, start.1, radius, [200, 0, 0]);
                        }
                    }
                    self.refresh_texture(&ctx);
                }

                self.drag_start = None;
                self.drag_current = None;
            }

            self.draw_preview(ui, response.rect);
        }
    }
}
