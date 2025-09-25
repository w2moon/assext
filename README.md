# Assext - Asset 文件扩展工具

这是一个用 Rust 编写的命令行工具，用于处理 Asset 文件，在指定区域生成数字。

## 功能

- 读取 Spine 文件（.atlas, .png, .skel）或单张图片文件
- 通过 GUI 窗口让用户选择图片上的矩形区域
- 在指定矩形区域内自动调整大小绘制数字
- 支持两种输出模式：
  - **多文件模式**：生成多个目录，每个目录包含完整的 Spine 文件
  - **单图片模式**：直接在输出目录下生成带编号的图片文件

## 使用方法

```bash
cargo build
./target/debug/assext <SPINE_PATH> <OUTPUT_DIR> <COUNT>
```

### 参数说明

- `SPINE_PATH`: Spine 文件路径（不包含扩展名）或单张图片路径，例如: `./data/lixiaolong` 或 `./datasingle/lixiaolong`
- `OUTPUT_DIR`: 输出目录，例如: `output`
- `COUNT`: 生成的文件数量，例如: `3`

### 使用模式

程序会根据输入文件自动选择输出模式：

#### 多文件模式（完整 Spine 文件）

当输入路径包含 `.atlas` 和/或 `.skel` 文件时，程序会使用多文件模式。

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

#### 单图片模式（仅 PNG 文件）

当输入路径只包含 `.png` 文件时，程序会使用单图片模式。

```bash
./target/debug/assext ./datasingle/lixiaolong output 3
```

这将：

1. 打开一个 GUI 窗口显示 `lixiaolong.png` 图片
2. 让用户在图片上拖拽选择矩形区域
3. 直接在 `output/` 目录下生成 3 个图片文件：
   - `lixiaolong_01.png`
   - `lixiaolong_02.png`
   - `lixiaolong_03.png`

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

## 更新日志

### v0.2.0

- 新增单图片模式支持
- 当输入目录只包含 PNG 文件时，直接在输出目录下生成带编号的图片文件
- 不再创建子目录，简化输出结构
- 自动检测输入文件类型并选择合适的输出模式

### v0.1.0

- 初始版本
- 支持完整的 Spine 文件处理
- GUI 矩形区域选择
- 多目录输出模式

## 注意事项

- 确保输入文件存在：
  - 多文件模式：需要 `.png` 文件，`.atlas` 和 `.skel` 文件可选
  - 单图片模式：只需要 `.png` 文件
- GUI 窗口需要在有图形界面的环境中运行
- 程序会自动调整文字大小以适应选择的矩形区域
- 生成的数字会居中显示在矩形区域内
- 程序会根据输入文件类型自动选择输出模式
- 数字格式：1-99 使用 2 位数字（01, 02, 03...），100+ 使用 3 位数字（001, 002, 003...）
