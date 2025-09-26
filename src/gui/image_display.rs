use crate::gui::{CoordinateCalculator, SelectionHandler};
use eframe::egui;

pub struct ImageDisplay {
    image_texture: Option<egui::TextureHandle>,
    image_size: egui::Vec2,
    loaded: bool,
}

impl ImageDisplay {
    pub fn new() -> Self {
        Self {
            image_texture: None,
            image_size: egui::Vec2::ZERO,
            loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn load_image(&mut self, ctx: &egui::Context, image_path: &str) {
        if let Ok(img) = image::open(image_path) {
            let rgb_img = img.to_rgb8();
            let size = [rgb_img.width() as usize, rgb_img.height() as usize];
            let pixels = rgb_img.into_raw();

            self.image_size = egui::Vec2::new(size[0] as f32, size[1] as f32);

            let color_image = egui::ColorImage::from_rgb(size, &pixels);
            self.image_texture =
                Some(ctx.load_texture("spine_image", color_image, Default::default()));
            self.loaded = true;
        } else {
        }
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        selection_handler: &mut SelectionHandler,
        _coordinate_calculator: &CoordinateCalculator,
    ) {
        if let Some(texture) = &self.image_texture {
            // 获取可用空间
            let available_size = ui.available_size();

            // 创建一个响应区域来显示图片并处理交互
            let image_response = ui.add(egui::Image::new(texture).max_width(available_size.x));

            // 获取图片的实际显示区域
            let actual_image_rect = image_response.rect;

            // 设置实际的图片显示区域到 selection_handler
            selection_handler.set_actual_image_rect(actual_image_rect);

            // 只有在启用选择框时才处理鼠标交互和绘制选择矩形
            if selection_handler.get_enable_selection() {
                // 处理鼠标交互 - 使用实际图片显示区域
                selection_handler.handle_mouse_interaction(ui, actual_image_rect);

                // 绘制选择矩形 - 使用实际图片显示区域
                selection_handler.draw_selection_rect(ui, actual_image_rect);
            }
        } else {
            ui.label("Loading image...");
        }
    }

    pub fn get_image_size(&self) -> egui::Vec2 {
        self.image_size
    }
}
