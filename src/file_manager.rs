use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct FileManager {
    output_dir: String,
    spine_name: String,
    is_single_image_mode: bool,
}

impl FileManager {
    pub fn new(output_dir: &str, spine_name: &str, is_single_image_mode: bool) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            spine_name: spine_name.to_string(),
            is_single_image_mode,
        }
    }

    pub fn create_output_dirs(&self, count: u32) -> Result<()> {
        // 创建输出根目录
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir)?;
        }

        // 如果是单图片模式，不需要创建子目录
        if self.is_single_image_mode {
            return Ok(());
        }

        // 创建每个子目录
        for i in 1..=count {
            // 根据数量决定数字格式：超过99个使用3位数字，否则使用2位数字
            let dir_name = if count > 99 {
                format!("{}_{:03}", self.spine_name, i)
            } else {
                format!("{}_{:02}", self.spine_name, i)
            };
            let dir_path = format!("{}/{}", self.output_dir, dir_name);

            if Path::new(&dir_path).exists() {
                fs::remove_dir_all(&dir_path)?;
            }
            fs::create_dir_all(&dir_path)?;
        }

        Ok(())
    }

    pub fn copy_files(
        &self,
        dir_name: &str,
        atlas_path: &str,
        skel_path: &str,
        has_atlas: bool,
        has_skel: bool,
    ) -> Result<()> {
        // 如果是单图片模式，不需要复制任何文件
        if self.is_single_image_mode {
            return Ok(());
        }

        let target_dir = format!("{}/{}", self.output_dir, dir_name);

        // 如果atlas文件存在，则复制
        if has_atlas {
            let atlas_target = format!("{}/{}.atlas", target_dir, self.spine_name);
            fs::copy(atlas_path, &atlas_target)?;
        }

        // 如果skel文件存在，则复制
        if has_skel {
            let skel_target = format!("{}/{}.skel", target_dir, self.spine_name);
            fs::copy(skel_path, &skel_target)?;
        }

        // PNG文件会在图片处理模块中处理，这里不需要复制

        Ok(())
    }

    // 保留旧方法以保持向后兼容
    pub fn copy_spine_files(
        &self,
        dir_name: &str,
        atlas_path: &str,
        _png_path: &str,
        skel_path: &str,
    ) -> Result<()> {
        self.copy_files(dir_name, atlas_path, skel_path, true, true)
    }
}
