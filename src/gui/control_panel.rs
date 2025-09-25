use crate::gui::{CoordinateCalculator, ImageDisplay, SelectionHandler};
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
                // 状态显示
                if selection_handler.is_selecting() {
                    ui.label("✅ 正在选择矩形区域...");
                }

                if selection_handler.has_selection() {
                    ui.label("✅ 已选择矩形区域，请点击下方按钮确认");
                } else {
                    ui.label("请先在图片上拖拽选择矩形区域");
                }

                ui.separator();

                // 颜色选择区域
                ui.label("文字颜色:");
                ui.horizontal(|ui| {
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

                ui.separator();

                // 按钮区域
                ui.vertical(|ui| {
                    if selection_handler.has_selection() {
                        if ui
                            .add_sized(
                                [ui.available_width(), 40.0],
                                egui::Button::new("✅ 确认选择"),
                            )
                            .clicked()
                        {
                            selection_handler.confirm_selection(
                                ctx,
                                coordinate_calculator,
                                image_display.get_image_size(),
                            );
                        }

                        if ui
                            .add_sized(
                                [ui.available_width(), 40.0],
                                egui::Button::new("🔄 重置选择"),
                            )
                            .clicked()
                        {
                            selection_handler.reset_selection();
                        }
                    }

                    if ui
                        .add_sized([ui.available_width(), 40.0], egui::Button::new("❌ 取消"))
                        .clicked()
                    {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.separator();

                // 显示当前选择信息
                if let Some((x, y, width, height)) = selection_handler
                    .get_selection_info(image_display.get_image_size())
                {
                    ui.label("选择区域信息:");
                    ui.label(format!("X: {:.0}", x));
                    ui.label(format!("Y: {:.0}", y));
                    ui.label(format!("宽度: {:.0}", width));
                    ui.label(format!("高度: {:.0}", height));
                }
            });
    }
}
