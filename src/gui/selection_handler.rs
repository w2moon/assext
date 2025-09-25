use crate::gui::{CoordinateCalculator, Rect};
use eframe::egui;
use std::sync::{Arc, Mutex};

pub struct SelectionHandler {
    is_selecting: bool,
    start_pos: Option<egui::Pos2>,
    current_pos: Option<egui::Pos2>,
    text_color: egui::Color32,
    selected_rect: Arc<Mutex<Option<Rect>>>,
}

impl SelectionHandler {
    pub fn new(selected_rect: Arc<Mutex<Option<Rect>>>) -> Self {
        Self {
            is_selecting: false,
            start_pos: None,
            current_pos: None,
            text_color: egui::Color32::BLACK,
            selected_rect,
        }
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

    pub fn draw_selection_rect(&self, ui: &mut egui::Ui, _image_rect: egui::Rect) {
        // 绘制选择矩形
        if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
            let rect = egui::Rect::from_two_pos(start, current);
            let painter = ui.painter();
            painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::RED));
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

    pub fn get_selection_info(
        &self,
        coordinate_calculator: &CoordinateCalculator,
        image_size: egui::Vec2,
    ) -> Option<(f32, f32, f32, f32)> {
        if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
            let rect = egui::Rect::from_two_pos(start, current);
            let available_size = egui::Vec2::new(800.0, 600.0);
            let (display_size, image_rect) =
                coordinate_calculator.calculate_image_display(available_size, image_size);

            let scale_x = image_size.x / display_size.x;
            let scale_y = image_size.y / display_size.y;

            let image_rect_x = ((rect.min.x - image_rect.min.x) * scale_x)
                .max(0.0)
                .min(image_size.x - 1.0);
            let image_rect_y = ((rect.min.y - image_rect.min.y) * scale_y)
                .max(0.0)
                .min(image_size.y - 1.0);
            let image_rect_width = (rect.width() * scale_x)
                .max(1.0)
                .min(image_size.x - image_rect_x);
            let image_rect_height = (rect.height() * scale_y)
                .max(1.0)
                .min(image_size.y - image_rect_y);

            Some((
                image_rect_x,
                image_rect_y,
                image_rect_width,
                image_rect_height,
            ))
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
}
