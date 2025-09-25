use crate::gui::{Rect, TextDirection};
use eframe::egui;

pub struct CoordinateCalculator;

impl CoordinateCalculator {
    pub fn new() -> Self {
        Self
    }

    // 计算图片在给定可用空间中的显示尺寸和位置
    pub fn calculate_image_display(
        &self,
        available_size: egui::Vec2,
        image_size: egui::Vec2,
    ) -> (egui::Vec2, egui::Rect) {
        // 留出一些边距
        let available_size = available_size - egui::Vec2::new(20.0, 20.0);
        let image_ratio = image_size.x / image_size.y;
        let available_ratio = available_size.x / available_size.y;

        // 计算显示尺寸，保持宽高比，确保图片完全适合可用区域
        let display_size = if image_ratio > available_ratio {
            // 图片更宽，以宽度为准
            egui::Vec2::new(available_size.x, available_size.x / image_ratio)
        } else {
            // 图片更高，以高度为准
            egui::Vec2::new(available_size.y * image_ratio, available_size.y)
        };

        // 计算图片在UI中的位置（左上角显示）
        let image_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), display_size);

        (display_size, image_rect)
    }

    // 将UI坐标转换为图片坐标
    pub fn ui_to_image_coords(
        &self,
        ui_rect: egui::Rect,
        image_rect: egui::Rect,
        display_size: egui::Vec2,
        image_size: egui::Vec2,
        text_color: egui::Color32,
    ) -> Rect {
        let scale_x = image_size.x / display_size.x;
        let scale_y = image_size.y / display_size.y;

        // 将UI坐标转换为图片坐标
        let image_rect_x = ((ui_rect.min.x - image_rect.min.x) * scale_x)
            .max(0.0)
            .min(image_size.x - 1.0);
        let image_rect_y = ((ui_rect.min.y - image_rect.min.y) * scale_y)
            .max(0.0)
            .min(image_size.y - 1.0);
        let image_rect_width = (ui_rect.width() * scale_x)
            .max(1.0)
            .min(image_size.x - image_rect_x);
        let image_rect_height = (ui_rect.height() * scale_y)
            .max(1.0)
            .min(image_size.y - image_rect_y);

        Rect {
            x: image_rect_x as i32,
            y: image_rect_y as i32,
            width: image_rect_width as u32,
            height: image_rect_height as u32,
            text_color,
            enable_color_variation: false,
            base_hue: 0.0,
            text_direction: TextDirection::Right,
        }
    }
}
