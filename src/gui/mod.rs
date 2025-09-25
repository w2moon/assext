pub mod control_panel;
pub mod coordinate_calculator;
pub mod image_display;
pub mod selection_handler;

pub use control_panel::ControlPanel;
pub use coordinate_calculator::CoordinateCalculator;
pub use image_display::ImageDisplay;
pub use selection_handler::SelectionHandler;

use anyhow::Result;
use eframe::egui;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub text_color: egui::Color32,
}

pub struct RectSelector {
    image_display: ImageDisplay,
    selection_handler: SelectionHandler,
    coordinate_calculator: CoordinateCalculator,
    control_panel: ControlPanel,
    selected_rect: Arc<Mutex<Option<Rect>>>,
    image_path: String,
}

impl RectSelector {
    pub fn select_rect(image_path: &str) -> Result<Rect> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 1000.0]),
            ..Default::default()
        };

        let selected_rect = Arc::new(Mutex::new(None));
        let selected_rect_clone = selected_rect.clone();
        let image_path = image_path.to_string();

        let selector = RectSelector {
            image_display: ImageDisplay::new(),
            selection_handler: SelectionHandler::new(selected_rect_clone.clone()),
            coordinate_calculator: CoordinateCalculator::new(),
            control_panel: ControlPanel::new(),
            selected_rect: selected_rect_clone.clone(),
            image_path,
        };

        eframe::run_native(
            "选择矩形区域",
            options,
            Box::new(move |cc| {
                setup_custom_fonts(&cc.egui_ctx);
                Box::new(selector)
            }),
        )
        .map_err(|e| anyhow::anyhow!("GUI错误: {}", e))?;

        let result = selected_rect.lock().unwrap().take();
        result.ok_or_else(|| anyhow::anyhow!("未选择矩形区域"))
    }
}

impl eframe::App for RectSelector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 加载图片（只在第一次调用时）
        if !self.image_display.is_loaded() {
            self.image_display.load_image(ctx, &self.image_path);
        }

        // 左侧面板 - 控制区域
        self.control_panel.show(
            ctx,
            &mut self.selection_handler,
            &self.coordinate_calculator,
            &self.image_display,
        );

        // 右侧面板 - 图片显示区域
        egui::CentralPanel::default().show(ctx, |ui| {
            self.image_display
                .show(ui, &mut self.selection_handler, &self.coordinate_calculator);
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 尝试加载中文字体
    let chinese_fonts = [
        "/System/Library/Fonts/PingFang.ttc",         // macOS 苹方字体
        "/System/Library/Fonts/STHeiti Light.ttc",    // macOS 黑体
        "/System/Library/Fonts/Hiragino Sans GB.ttc", // macOS 冬青黑体
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", // Linux
        "C:\\Windows\\Fonts\\msyh.ttc",               // Windows 微软雅黑
        "C:\\Windows\\Fonts\\simhei.ttf",             // Windows 黑体
    ];

    for font_path in &chinese_fonts {
        if std::path::Path::new(font_path).exists() {
            if let Ok(font_data) = std::fs::read(font_path) {
                fonts.font_data.insert(
                    "chinese_font".to_owned(),
                    egui::FontData::from_owned(font_data),
                );

                // 设置字体族
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "chinese_font".to_owned());

                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .push("chinese_font".to_owned());

                println!("成功加载中文字体: {}", font_path);
                break;
            }
        }
    }

    ctx.set_fonts(fonts);
}
