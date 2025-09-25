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
        println!("正在加载图片: {}", image_path);
        if let Ok(img) = image::open(image_path) {
            let rgb_img = img.to_rgb8();
            let size = [rgb_img.width() as usize, rgb_img.height() as usize];
            let pixels = rgb_img.into_raw();

            self.image_size = egui::Vec2::new(size[0] as f32, size[1] as f32);
            println!("图片尺寸: {}x{}", size[0], size[1]);

            let color_image = egui::ColorImage::from_rgb(size, &pixels);
            self.image_texture =
                Some(ctx.load_texture("spine_image", color_image, Default::default()));
            println!("图片加载完成");
            self.loaded = true;
        } else {
            println!("无法加载图片: {}", image_path);
        }
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        selection_handler: &mut SelectionHandler,
        coordinate_calculator: &CoordinateCalculator,
    ) {
        if let Some(texture) = &self.image_texture {
            // 获取可用空间
            let available_size = ui.available_size();
            let window_size = ui.ctx().screen_rect().size();
            println!("右侧面板可用空间: {:?}", available_size);
            println!("整个窗口尺寸: {:?}", window_size);

            // 使用统一的计算方法
            let (display_size, image_rect) =
                coordinate_calculator.calculate_image_display(available_size, self.image_size);
            println!("计算出的显示尺寸: {:?}", display_size);
            println!("图片矩形位置: {:?}", image_rect);

            // 显示图片，使用计算出的尺寸
            // ui.allocate_ui_at_rect(image_rect, |ui| {
            //     ui.image(texture);
            // });
            // ui.image(texture);

            ui.add(egui::Image::new(texture).max_width(available_size.x));

            // 处理鼠标交互
            // selection_handler.handle_mouse_interaction(ui, image_rect);

            // 绘制选择矩形
            // selection_handler.draw_selection_rect(ui, image_rect);
        } else {
            ui.label("正在加载图片...");
        }
    }

    pub fn get_image_size(&self) -> egui::Vec2 {
        self.image_size
    }
}
