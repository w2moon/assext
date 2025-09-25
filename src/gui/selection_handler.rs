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
                println!("开始选择: {:?}", pointer_pos);
            }
        }

        // 支持直接拖拽开始选择
        if response.dragged() {
            if let Some(pointer_pos) = response.interact_pointer_pos() {
                if !self.is_selecting {
                    // 如果还没有开始选择，现在开始
                    self.is_selecting = true;
                    self.start_pos = Some(pointer_pos);
                    println!("直接拖拽开始选择: {:?}", pointer_pos);
                }
                self.current_pos = Some(pointer_pos);
                println!("拖拽中: {:?}", pointer_pos);
            }
        }

        // 拖拽释放时结束选择
        if response.drag_released() && self.is_selecting {
            self.is_selecting = false;
            println!("拖拽结束，完成选择");
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
        coordinate_calculator: &CoordinateCalculator,
        image_size: egui::Vec2,
    ) {
        if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
            println!("开始确认选择: start={:?}, current={:?}", start, current);

            // 使用与显示时相同的计算方法
            let available_size = egui::Vec2::new(800.0, 600.0); // 估计的右侧面板大小
            let (display_size, image_rect) =
                coordinate_calculator.calculate_image_display(available_size, image_size);

            println!("显示尺寸: {:?}, 图片矩形: {:?}", display_size, image_rect);

            let rect = egui::Rect::from_two_pos(start, current);
            println!("选择矩形: {:?}", rect);

            // 检查选择是否在图片区域内
            if !image_rect.contains(rect.min) || !image_rect.contains(rect.max) {
                println!("选择区域超出图片范围，调整坐标");
                // 将坐标限制在图片区域内
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
                println!("调整后的矩形: {:?}", clamped_rect);

                let selected_rect = coordinate_calculator.ui_to_image_coords(
                    clamped_rect,
                    image_rect,
                    display_size,
                    image_size,
                    self.text_color,
                );
                println!("确认选择: {:?}", selected_rect);
                *self.selected_rect.lock().unwrap() = Some(selected_rect);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                println!("选择区域在图片范围内");
                let selected_rect = coordinate_calculator.ui_to_image_coords(
                    rect,
                    image_rect,
                    display_size,
                    image_size,
                    self.text_color,
                );
                println!("确认选择: {:?}", selected_rect);
                *self.selected_rect.lock().unwrap() = Some(selected_rect);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        } else {
            println!(
                "没有选择区域: start={:?}, current={:?}",
                self.start_pos, self.current_pos
            );
        }
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
        println!("get_selection_info - image_size: {:?}", image_size);

        if let Some(actual_image_rect) = self.actual_image_rect {
            println!(
                "get_selection_info - actual_image_rect: {:?}",
                actual_image_rect
            );

            if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
                let selection_rect = egui::Rect::from_two_pos(start, current);
                println!("get_selection_info - selection_rect: {:?}", selection_rect);

                // 计算缩放比例：原始图片尺寸 / 实际显示尺寸
                let scale_x = image_size.x / actual_image_rect.width();
                let scale_y = image_size.y / actual_image_rect.height();

                println!(
                    "get_selection_info - scale_x: {}, scale_y: {}",
                    scale_x, scale_y
                );

                // 将选择框坐标转换为相对于图片显示区域的坐标
                let relative_x = (selection_rect.min.x - actual_image_rect.min.x).max(0.0);
                let relative_y = (selection_rect.min.y - actual_image_rect.min.y).max(0.0);
                let relative_width = selection_rect
                    .width()
                    .min(actual_image_rect.width() - relative_x);
                let relative_height = selection_rect
                    .height()
                    .min(actual_image_rect.height() - relative_y);

                println!(
                    "get_selection_info - 相对坐标: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
                    relative_x, relative_y, relative_width, relative_height
                );

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

                println!(
                    "get_selection_info - 最终结果: x={:.1}, y={:.1}, w={:.1}, h={:.1}",
                    result.0, result.1, result.2, result.3
                );

                Some(result)
            } else {
                None
            }
        } else {
            println!("get_selection_info - 没有设置 actual_image_rect");
            None
        }
    }

    pub fn set_text_color(&mut self, color: egui::Color32) {
        self.text_color = color;
    }

    pub fn get_text_color(&self) -> egui::Color32 {
        self.text_color
    }
}
