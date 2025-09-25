use clap::Parser;
use std::path::Path;
use anyhow::Result;

mod gui;
mod image_processor;
mod file_manager;

use gui::RectSelector;
use image_processor::ImageProcessor;
use file_manager::FileManager;

#[derive(Parser)]
#[command(name = "spext")]
#[command(about = "Spine文件扩展工具 - 在指定区域生成数字")]
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
    
    println!("Spine文件扩展工具启动...");
    println!("Spine路径: {}", args.spine_path);
    println!("输出目录: {}", args.output_dir);
    println!("生成数量: {}", args.count);
    
    // 检查Spine文件是否存在
    let atlas_path = format!("{}.atlas", args.spine_path);
    let png_path = format!("{}.png", args.spine_path);
    let skel_path = format!("{}.skel", args.spine_path);
    
    if !Path::new(&atlas_path).exists() {
        anyhow::bail!("Atlas文件不存在: {}", atlas_path);
    }
    if !Path::new(&png_path).exists() {
        anyhow::bail!("PNG文件不存在: {}", png_path);
    }
    if !Path::new(&skel_path).exists() {
        anyhow::bail!("Skel文件不存在: {}", skel_path);
    }
    
    // 获取文件名（不包含路径）
    let spine_name = Path::new(&args.spine_path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    
    // 打开GUI选择矩形区域
    println!("正在打开图片选择窗口...");
    
    // 临时测试：使用固定矩形区域
    let rect = if std::env::args().any(|arg| arg == "--test") {
        println!("使用测试模式，固定矩形区域");
        gui::Rect { 
            x: 100, 
            y: 100, 
            width: 200, 
            height: 100,
            text_color: eframe::egui::Color32::BLACK
        }
    } else {
        RectSelector::select_rect(&png_path)?
    };
    
    println!("选择的矩形区域: x={}, y={}, width={}, height={}", 
             rect.x, rect.y, rect.width, rect.height);
    
    // 创建输出目录
    let file_manager = FileManager::new(&args.output_dir, &spine_name);
    file_manager.create_output_dirs(args.count)?;
    
    // 处理每个文件
    let image_processor = ImageProcessor::new(&png_path);
    
    for i in 1..=args.count {
        let dir_name = format!("{}_{:02}", spine_name, i);
        let number_text = format!("{:02}", i);
        
        println!("处理目录: {}", dir_name);
        
        // 复制文件
        file_manager.copy_spine_files(&dir_name, &atlas_path, &png_path, &skel_path)?;
        
        // 在PNG上绘制数字
        let output_png_path = format!("{}/{}/{}.png", args.output_dir, dir_name, spine_name);
        image_processor.draw_text_in_rect(&output_png_path, &number_text, &rect)?;
    }
    
    println!("处理完成！生成了 {} 个目录。", args.count);
    Ok(())
}