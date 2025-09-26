# Assext - Asset File Extension Tool

A command-line tool written in Rust for processing Asset files and generating numbers in specified regions.

## Features

- Read Spine files (.atlas, .png, .skel) or single image files
- Allow users to select rectangular regions on images through a GUI window
- Automatically adjust and draw numbers within specified rectangular regions
- Support two output modes:
  - **Multi-file mode**: Generate multiple directories, each containing complete Spine files
  - **Single-image mode**: Generate numbered image files directly in the output directory

## Installation

```bash
cargo install assext
```

## Usage

```bash
assext <SPINE_PATH> <OUTPUT_DIR> <COUNT>
```

### Parameters

- `SPINE_PATH`: Spine file path (without extension) or single image path, e.g., `./data/lixiaolong` or `./datasingle/lixiaolong`
- `OUTPUT_DIR`: Output directory, e.g., `output`
- `COUNT`: Number of files to generate, e.g., `3`

### Usage Modes

The program automatically selects the output mode based on input files:

#### Multi-file Mode (Complete Spine Files)

When the input path contains `.atlas` and/or `.skel` files, the program uses multi-file mode.

```bash
assext ./data/lixiaolong output 3
```

This will:

1. Open a GUI window displaying the `lixiaolong.png` image
2. Allow users to drag and select a rectangular region on the image
3. Create 3 subdirectories in the `output/` directory:
   - `lixiaolong_01/`
   - `lixiaolong_02/`
   - `lixiaolong_03/`
4. Each directory contains:
   - `lixiaolong.atlas` (copied original file)
   - `lixiaolong.png` (image with corresponding number drawn in the specified region)
   - `lixiaolong.skel` (copied original file)

#### Single-image Mode (PNG Files Only)

When the input path contains only `.png` files, the program uses single-image mode.

```bash
assext ./datasingle/lixiaolong output 3
```

This will:

1. Open a GUI window displaying the `lixiaolong.png` image
2. Allow users to drag and select a rectangular region on the image
3. Generate 3 image files directly in the `output/` directory:
   - `lixiaolong_01.png`
   - `lixiaolong_02.png`
   - `lixiaolong_03.png`

## GUI Usage Instructions

1. The program will open a window displaying the Spine image upon startup
2. Drag the mouse on the image to select a rectangular region
3. A red border will show the currently selected region
4. Click the "Confirm" button to confirm the selection
5. Click the "Cancel" button to exit the program

## Dependencies

- Rust 1.70+
- System fonts (Arial, Helvetica, etc.)

## Building from Source

```bash
git clone <repository-url>
cd assext
cargo build --release
```

## Changelog

### v0.2.0

- Added single-image mode support
- When input directory contains only PNG files, generate numbered image files directly in the output directory
- No longer creates subdirectories, simplifying output structure
- Automatically detect input file types and select appropriate output mode

### v0.1.0

- Initial version
- Support for complete Spine file processing
- GUI rectangular region selection
- Multi-directory output mode

## Notes

- Ensure input files exist:
  - Multi-file mode: Requires `.png` file, `.atlas` and `.skel` files are optional
  - Single-image mode: Only requires `.png` file
- GUI window needs to run in an environment with a graphical interface
- The program automatically adjusts text size to fit the selected rectangular region
- Generated numbers are centered within the rectangular region
- The program automatically selects output mode based on input file types
- Number format: 1-99 uses 2-digit format (01, 02, 03...), 100+ uses 3-digit format (001, 002, 003...)
