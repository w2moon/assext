use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod file_manager;
mod gui;
mod image_processor;

use file_manager::FileManager;
use gui::RectSelector;
use image_processor::ImageProcessor;

#[derive(Parser)]
#[command(name = "assext")]
#[command(about = "Asset文件扩展工具 - 在指定区域生成数字")]
struct Args {
    /// Spine文件路径（不包含扩展名）
    #[arg(help = "Spine文件路径，例如: ./data/lixiaolong")]
    spine_path: String,

    /// 输出目录
    #[arg(help = "输出目录，例如: output")]
    output_dir: String,

    /// 生成数量
    #[arg(help = "生成的文件数量，例如: 3")]
    count: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 检查文件是否存在
    let atlas_path = format!("{}.atlas", args.spine_path);
    let png_path = format!("{}.png", args.spine_path);
    let skel_path = format!("{}.skel", args.spine_path);

    // PNG文件是必需的
    if !Path::new(&png_path).exists() {
        anyhow::bail!("PNG文件不存在: {}", png_path);
    }

    // 检查其他文件是否存在
    let has_atlas = Path::new(&atlas_path).exists();
    let has_skel = Path::new(&skel_path).exists();

    // 检测是否为单图片模式（只有PNG文件，没有atlas和skel文件）
    let is_single_image_mode = !has_atlas && !has_skel;

    // 获取文件名（不包含路径）
    let spine_name = Path::new(&args.spine_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // 打开GUI选择矩形区域

    // 临时测试：使用固定矩形区域
    let rect = RectSelector::select_rect(&png_path)?;

    println!(
        "选择的矩形区域: x={}, y={}, width={}, height={}",
        rect.x, rect.y, rect.width, rect.height
    );

    // 创建输出目录
    let file_manager = FileManager::new(&args.output_dir, &spine_name, is_single_image_mode);
    file_manager.create_output_dirs(args.count)?;

    // 处理每个文件
    let image_processor = ImageProcessor::new(&png_path);

    for i in 1..=args.count {
        // 根据数量决定数字格式：超过99个使用3位数字，否则使用2位数字
        let (dir_name, number_text) = if args.count > 99 {
            // 3位数字格式：001, 002, 003...
            let formatted_num = format!("{:03}", i);
            (format!("{}_{}", spine_name, formatted_num), formatted_num)
        } else {
            // 2位数字格式：01, 02, 03...
            let formatted_num = format!("{:02}", i);
            (format!("{}_{}", spine_name, formatted_num), formatted_num)
        };

        // 复制文件（在非单图片模式下）
        file_manager.copy_files(&dir_name, &atlas_path, &skel_path, has_atlas, has_skel)?;

        // 在PNG上绘制数字
        let output_png_path = if is_single_image_mode {
            // 单图片模式：直接在output目录下生成带编号的图片
            format!("{}/{}_{}.png", args.output_dir, spine_name, number_text)
        } else {
            // 多文件模式：在子目录中生成图片
            format!("{}/{}/{}.png", args.output_dir, dir_name, spine_name)
        };

        image_processor.draw_text_in_rect_with_color_variation(
            &output_png_path,
            &number_text,
            &rect,
            rect.enable_color_variation,
            0.0, // base_hue 不再使用，传递0.0
            i,
        )?;
    }

    if is_single_image_mode {
        println!("处理完成！在output目录下生成了 {} 个图片文件。", args.count);
    } else {
        println!("处理完成！生成了 {} 个目录。", args.count);
    }
    Ok(())
}
