use crate::gui::{CoordinateCalculator, ImageDisplay, SelectionHandler, TextDirection};
use eframe::egui;

pub struct ControlPanel;

impl ControlPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        selection_handler: &mut SelectionHandler,
        coordinate_calculator: &CoordinateCalculator,
        image_display: &ImageDisplay,
    ) {
        // 左侧面板 - 控制区域
        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(300.0)
            .min_width(250.0)
            .max_width(400.0)
            .show(ctx, |ui| {
                // 获取可用宽度，确保所有元素使用一致的宽度
                let available_width = ui.available_width();
                let button_width = (available_width - 20.0) / 2.0; // 减去间距

                // 上部：确认和取消按钮
                ui.group(|ui| {
                    ui.set_min_height(100.0); // 设置固定高度，更合适
                    ui.vertical_centered(|ui| {
                        ui.heading("Operations");
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            if ui
                                .add_sized([button_width, 40.0], egui::Button::new("✅ Confirm"))
                                .clicked()
                            {
                                selection_handler.confirm_selection(
                                    ctx,
                                    coordinate_calculator,
                                    image_display.get_image_size(),
                                );
                            }

                            if ui
                                .add_sized([button_width, 40.0], egui::Button::new("❌ Cancel"))
                                .clicked()
                            {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                });

                ui.add_space(10.0);

                // 中部：选择框区域
                ui.group(|ui| {
                    ui.set_min_height(ui.available_height() * 0.25);
                    ui.vertical(|ui| {
                        ui.heading("Selection Area");
                        ui.add_space(10.0);

                        // 选择框开关
                        ui.horizontal(|ui| {
                            ui.set_min_width(available_width - 20.0); // 设置最小宽度保持一致
                            let mut enable_selection = selection_handler.get_enable_selection();
                            ui.checkbox(&mut enable_selection, "Enable Selection");
                            selection_handler.set_enable_selection(enable_selection);
                        });

                        if selection_handler.get_enable_selection() {
                            ui.add_space(10.0);

                            // 状态显示
                            if selection_handler.is_selecting() {
                                ui.label("✅ Selecting rectangle region...");
                            } else if selection_handler.has_selection() {
                                ui.label("✅ Rectangle region selected");
                            } else {
                                ui.label("Please drag on the image to select a rectangle region");
                            }

                            // 显示当前选择信息
                            if let Some((x, y, width, height)) =
                                selection_handler.get_selection_info(image_display.get_image_size())
                            {
                                ui.add_space(5.0);
                                ui.label("Selection area info:");
                                ui.label(format!("X: {:.0}, Y: {:.0}", x, y));
                                ui.label(format!("Width: {:.0}, Height: {:.0}", width, height));
                            }

                            ui.add_space(10.0);

                            // 文字朝向选择
                            ui.horizontal(|ui| {
                                ui.label("Text Direction:");
                                let mut current_direction = selection_handler.get_text_direction();
                                egui::ComboBox::from_id_source("text_direction")
                                    .width(available_width - 100.0) // 设置固定宽度
                                    .selected_text(current_direction.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Up,
                                            "Up",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Down,
                                            "Down",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Left,
                                            "Left",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Right,
                                            "Right",
                                        );
                                    });
                                selection_handler.set_text_direction(current_direction);
                            });
                        } else {
                            ui.add_space(10.0);
                            ui.label("Will use the entire image area");

                            ui.add_space(10.0);

                            // 文字朝向选择（即使没有选择框也显示）
                            ui.horizontal(|ui| {
                                ui.label("Text Direction:");
                                let mut current_direction = selection_handler.get_text_direction();
                                egui::ComboBox::from_id_source("text_direction")
                                    .width(available_width - 100.0) // 设置固定宽度
                                    .selected_text(current_direction.as_str())
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Up,
                                            "Up",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Down,
                                            "Down",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Left,
                                            "Left",
                                        );
                                        ui.selectable_value(
                                            &mut current_direction,
                                            TextDirection::Right,
                                            "Right",
                                        );
                                    });
                                selection_handler.set_text_direction(current_direction);
                            });
                        }
                    });
                });

                ui.add_space(10.0);

                // 文字颜色选择区域
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Text Color");
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.set_min_width(available_width - 20.0); // 设置最小宽度保持一致
                            let mut text_color = selection_handler.get_text_color();
                            egui::color_picker::color_picker_color32(
                                ui,
                                &mut text_color,
                                egui::color_picker::Alpha::Opaque,
                            );
                            selection_handler.set_text_color(text_color);
                            ui.label(format!(
                                "RGB({}, {}, {})",
                                text_color.r(),
                                text_color.g(),
                                text_color.b()
                            ));
                        });
                    });
                });

                ui.add_space(10.0);

                // 下部：颜色变化区域
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Color Variation");
                        ui.add_space(10.0);

                        // 颜色变化开关
                        ui.horizontal(|ui| {
                            ui.set_min_width(available_width - 20.0); // 设置最小宽度保持一致
                            let mut enable_color_variation =
                                selection_handler.get_enable_color_variation();
                            ui.checkbox(&mut enable_color_variation, "Enable Color Variation");
                            selection_handler.set_enable_color_variation(enable_color_variation);
                        });

                        if selection_handler.get_enable_color_variation() {
                            ui.add_space(10.0);
                            ui.label("Each generated image will automatically overlay different HSL colors");
                            ui.label("Colors will be evenly distributed based on the number of images");
                        } else {
                            ui.add_space(10.0);
                            ui.label("Will use the original image colors");
                        }
                    });
                });
            });
    }
}
