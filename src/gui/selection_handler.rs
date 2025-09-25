use crate::gui::{CoordinateCalculator, Rect};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct SelectionHandler {
    is_selecting: bool,
    start_pos: Option<egui::Pos2>,
    current_pos: Option<egui::Pos2>,
    text_color: egui::Color32,
    selected_rect: Arc<Mutex<Option<Rect>>>,
    actual_image_rect: Option<egui::Rect>,
    // 新增字段
    enable_selection: bool,
    enable_color_variation: bool,
}

impl SelectionHandler {
    pub fn new(selected_rect: Arc<Mutex<Option<Rect>>>) -> Self {
        Self {
            is_selecting: false,
            start_pos: None,
            current_pos: None,
            text_color: egui::Color32::BLACK,
            selected_rect,
            actual_image_rect: None,
            enable_selection: true,
            enable_color_variation: true,
        }
    }

    pub fn set_actual_image_rect(&mut self, rect: egui::Rect) {
        self.actual_image_rect = Some(rect);
    }

    pub fn handle_mouse_interaction(&mut self, ui: &mut egui::Ui, image_rect: egui::Rect) {
        // 处理鼠标交互 - 支持直接拖拽
        let response = ui.interact(
            image_rect,
            egui::Id::new("image"),
            egui::Sense::click_and_drag(),
        );

        // 检查鼠标事件
        if response.clicked() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                self.is_selecting = true;
                self.start_pos = Some(pointer_pos);
                self.current_pos = Some(pointer_pos);
            }
        }

        // 支持直接拖拽开始选择
        if response.dragged() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                if !self.is_selecting {
                    // 如果还没有开始选择，现在开始
                    self.is_selecting = true;
                    self.start_pos = Some(pointer_pos);
                }
                self.current_pos = Some(pointer_pos);
            }
        }

        // 拖拽释放时结束选择
        if response.drag_released() && self.is_selecting {
            self.is_selecting = false;
        }
    }

    pub fn draw_selection_rect(&self, ui: &mut egui::Ui, image_rect: egui::Rect) {
        // 绘制选择矩形
        if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
            let rect = egui::Rect::from_two_pos(start, current);

            // 确保选取框在图片区域内
            let clamped_rect = egui::Rect::from_min_max(
                egui::Pos2::new(
                    rect.min.x.max(image_rect.min.x).min(image_rect.max.x),
                    rect.min.y.max(image_rect.min.y).min(image_rect.max.y),
                ),
                egui::Pos2::new(
                    rect.max.x.max(image_rect.min.x).min(image_rect.max.x),
                    rect.max.y.max(image_rect.min.y).min(image_rect.max.y),
                ),
            );

            let painter = ui.painter();
            painter.rect_stroke(
                clamped_rect,
                0.0,
                egui::Stroke::new(2.0, egui::Color32::RED),
            );
        }
    }

    pub fn confirm_selection(
        &mut self,
        ctx: &egui::Context,
        _coordinate_calculator: &CoordinateCalculator,
        image_size: egui::Vec2,
    ) {
        if self.enable_selection {
            // 使用 get_selection_info 获取计算好的矩形坐标
            if let Some((x, y, width, height)) = self.get_selection_info(image_size) {
                // 创建 Rect 结构体
                let selected_rect = crate::gui::Rect {
                    x: x as i32,
                    y: y as i32,
                    width: width as u32,
                    height: height as u32,
                    text_color: self.text_color,
                    enable_color_variation: self.enable_color_variation,
                    base_hue: 0.0, // 不再使用，设为0.0
                };
                *self.selected_rect.lock().unwrap() = Some(selected_rect);
            }
        } else {
            // 如果没有启用选择，创建一个默认的矩形（整个图片）
            let default_rect = crate::gui::Rect {
                x: 0,
                y: 0,
                width: image_size.x as u32,
                height: image_size.y as u32,
                text_color: self.text_color,
                enable_color_variation: self.enable_color_variation,
                base_hue: 0.0, // 不再使用，设为0.0
            };
            *self.selected_rect.lock().unwrap() = Some(default_rect);
        }
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    pub fn reset_selection(&mut self) {
        self.is_selecting = false;
        self.start_pos = None;
        self.current_pos = None;
    }

    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    pub fn has_selection(&self) -> bool {
        self.start_pos.is_some() && self.current_pos.is_some()
    }

    pub fn get_selection_info(&self, image_size: egui::Vec2) -> Option<(f32, f32, f32, f32)> {
        if let Some(actual_image_rect) = self.actual_image_rect {
            if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
                let selection_rect = egui::Rect::from_two_pos(start, current);

                // 计算缩放比例：原始图片尺寸 / 实际显示尺寸
                let scale_x = image_size.x / actual_image_rect.width();
                let scale_y = image_size.y / actual_image_rect.height();

                // 将选择框坐标转换为相对于图片显示区域的坐标
                let relative_x = (selection_rect.min.x - actual_image_rect.min.x).max(0.0);
                let relative_y = (selection_rect.min.y - actual_image_rect.min.y).max(0.0);
                let relative_width = selection_rect
                    .width()
                    .min(actual_image_rect.width() - relative_x);
                let relative_height = selection_rect
                    .height()
                    .min(actual_image_rect.height() - relative_y);

                // 转换为原始图片坐标
                let image_x = (relative_x * scale_x).max(0.0).min(image_size.x - 1.0);
                let image_y = (relative_y * scale_y).max(0.0).min(image_size.y - 1.0);
                let image_width = (relative_width * scale_x)
                    .max(1.0)
                    .min(image_size.x - image_x);
                let image_height = (relative_height * scale_y)
                    .max(1.0)
                    .min(image_size.y - image_y);

                let result = (image_x, image_y, image_width, image_height);

                Some(result)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_text_color(&mut self, color: egui::Color32) {
        self.text_color = color;
    }

    pub fn get_text_color(&self) -> egui::Color32 {
        self.text_color
    }

    // 新增方法
    pub fn set_enable_selection(&mut self, enable: bool) {
        self.enable_selection = enable;
    }

    pub fn get_enable_selection(&self) -> bool {
        self.enable_selection
    }

    pub fn set_enable_color_variation(&mut self, enable: bool) {
        self.enable_color_variation = enable;
    }

    pub fn get_enable_color_variation(&self) -> bool {
        self.enable_color_variation
    }
}
