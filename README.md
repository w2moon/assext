# Assext - Asset 文件扩展工具

这是一个用 Rust 编写的命令行工具，用于处理 Asset 文件，在指定区域生成数字。

## 功能

- 读取 Spine 文件（.atlas, .png, .skel）
- 通过 GUI 窗口让用户选择图片上的矩形区域
- 在指定矩形区域内自动调整大小绘制数字
- 生成多个目录，每个目录包含修改后的 Spine 文件

## 使用方法

```bash
cargo build
./target/debug/assext <SPINE_PATH> <OUTPUT_DIR> <COUNT>
```

### 参数说明

- `SPINE_PATH`: Spine 文件路径（不包含扩展名），例如: `./data/lixiaolong`
- `OUTPUT_DIR`: 输出目录，例如: `output`
- `COUNT`: 生成的文件数量，例如: `3`

### 示例

```bash
./target/debug/assext ./data/lixiaolong output 3
```

这将：

1. 打开一个 GUI 窗口显示 `lixiaolong.png` 图片
2. 让用户在图片上拖拽选择矩形区域
3. 在 `output/` 目录下创建 3 个子目录：
   - `lixiaolong_01/`
   - `lixiaolong_02/`
   - `lixiaolong_03/`
4. 每个目录包含：
   - `lixiaolong.atlas` (复制的原文件)
   - `lixiaolong.png` (在指定区域绘制了对应数字的图片)
   - `lixiaolong.skel` (复制的原文件)

## GUI 使用说明

1. 程序启动后会打开一个窗口显示 Spine 图片
2. 在图片上拖拽鼠标选择矩形区域
3. 红色边框会显示当前选择的区域
4. 点击"确认选择"按钮确认选择
5. 点击"取消"按钮退出程序

## 依赖

- Rust 1.70+
- 系统字体（Arial, Helvetica 等）

## 构建

```bash
cargo build --release
```

## 注意事项

- 确保 Spine 文件（.atlas, .png, .skel）存在于指定路径
- GUI 窗口需要在有图形界面的环境中运行
- 程序会自动调整文字大小以适应选择的矩形区域
- 生成的数字会居中显示在矩形区域内
