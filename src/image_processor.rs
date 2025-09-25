use crate::gui::Rect;
use anyhow::Result;
use image::{DynamicImage, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::path::Path;

pub struct ImageProcessor {
    original_image: DynamicImage,
}

impl ImageProcessor {
    pub fn new(image_path: &str) -> Self {
        let img = image::open(image_path).expect("无法打开图片");
        Self {
            original_image: img,
        }
    }

    pub fn draw_text_in_rect(&self, output_path: &str, text: &str, rect: &Rect) -> Result<()> {
        self.draw_text_in_rect_with_color_variation(output_path, text, rect, false, 0.0, 0)
    }

    pub fn draw_text_in_rect_with_color_variation(
        &self,
        output_path: &str,
        text: &str,
        rect: &Rect,
        enable_color_variation: bool,
        base_hue: f32,
        index: u32,
    ) -> Result<()> {
        // 克隆原始图片
        let img = self.original_image.clone();

        // 转换为RGBA格式以便绘制
        let mut rgba_img = img.to_rgba8();

        // 计算文字大小，使其适合矩形区域
        let font_size = self.calculate_font_size(text, rect.width, rect.height);

        // 加载系统字体
        let font = self
            .load_system_font()
            .ok_or_else(|| anyhow::anyhow!("无法加载字体"))?;

        // 计算文字位置（居中）
        let scale = Scale::uniform(font_size);
        let text_width = self.measure_text_width(text, &font, scale);
        let text_height = font.v_metrics(scale).ascent - font.v_metrics(scale).descent;

        let text_x = rect.x + (rect.width as i32 - text_width as i32) / 2;
        let text_y = rect.y + (rect.height as i32 - text_height as i32) / 2;

        // 绘制文字
        let color = rect.text_color;

        draw_text_mut(
            &mut rgba_img,
            Rgba([color.r(), color.g(), color.b(), 255]), // 使用选择的颜色
            text_x,
            text_y,
            scale,
            &font,
            text,
        );

        // 应用颜色变化
        if enable_color_variation {
            self.apply_color_variation(&mut rgba_img, base_hue, index);
        }

        // 保存图片
        let final_img = DynamicImage::ImageRgba8(rgba_img);
        final_img.save(output_path)?;

        Ok(())
    }

    fn calculate_font_size(&self, text: &str, rect_width: u32, rect_height: u32) -> f32 {
        // 根据矩形大小和文字长度计算合适的字体大小
        let text_len = text.len() as f32;
        let width_ratio = rect_width as f32 / (text_len * 0.6); // 假设每个字符宽度约为字体大小的0.6倍
        let height_ratio = rect_height as f32 * 0.8; // 使用矩形高度的80%

        // 取较小的值作为字体大小，确保文字能完全放入矩形内
        let font_size = width_ratio.min(height_ratio);

        // 限制字体大小范围
        font_size.max(12.0).min(200.0)
    }

    fn measure_text_width(&self, text: &str, font: &Font, scale: Scale) -> f32 {
        let mut width: f32 = 0.0;
        for glyph in font.layout(text, scale, rusttype::point(0.0, 0.0)) {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                width = width.max(bounding_box.max.x as f32);
            }
        }
        width
    }

    fn load_system_font(&self) -> Option<Font<'static>> {
        // 尝试加载系统字体
        // 这里可以添加更多系统字体路径
        let system_fonts = [
            "/System/Library/Fonts/Arial.ttf",                 // macOS
            "/System/Library/Fonts/Helvetica.ttc",             // macOS
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", // Linux
            "C:\\Windows\\Fonts\\arial.ttf",                   // Windows
        ];

        for font_path in &system_fonts {
            if Path::new(font_path).exists() {
                if let Ok(font_data) = std::fs::read(font_path) {
                    // 将字体数据转换为静态生命周期
                    let font_data = Box::leak(font_data.into_boxed_slice());
                    if let Some(font) = Font::try_from_bytes(font_data) {
                        return Some(font);
                    }
                }
            }
        }

        None
    }

    fn apply_color_variation(&self, rgba_img: &mut image::RgbaImage, _base_hue: f32, index: u32) {
        // 根据图片索引自动生成色调，均匀分布在360度色环上
        // 使用黄金比例来获得更好的颜色分布
        let golden_ratio = 1.618033988749895;
        let current_hue = (index as f32 * 360.0 * golden_ratio) % 360.0;

        // 将色调转换为弧度
        let hue_rad = current_hue.to_radians();

        // 计算RGB通道的色调偏移

        // 对每个像素应用颜色变化
        for pixel in rgba_img.pixels_mut() {
            let r = pixel[0] as f32 / 255.0;
            let g = pixel[1] as f32 / 255.0;
            let b = pixel[2] as f32 / 255.0;

            // 计算亮度（用于保持原始亮度）
            let brightness = (r + g + b) / 3.0;

            // 如果像素太暗或太亮，跳过处理
            if brightness < 0.1 || brightness > 0.9 {
                continue;
            }

            // 应用色调变化到RGB通道
            // 使用不同的相位偏移来创建更丰富的颜色变化
            let r_phase = hue_rad;
            let g_phase = hue_rad + 2.0943951023931953; // +120度
            let b_phase = hue_rad + 4.1887902047863905; // +240度

            // 计算新的RGB值，保持原始亮度
            let new_r = (brightness + 0.3 * r_phase.cos()).clamp(0.0, 1.0);
            let new_g = (brightness + 0.3 * g_phase.cos()).clamp(0.0, 1.0);
            let new_b = (brightness + 0.3 * b_phase.cos()).clamp(0.0, 1.0);

            // 应用颜色变化，但保持原始像素的透明度
            pixel[0] = (new_r * 255.0) as u8;
            pixel[1] = (new_g * 255.0) as u8;
            pixel[2] = (new_b * 255.0) as u8;
            // pixel[3] 保持原始alpha值不变
        }
    }
}
