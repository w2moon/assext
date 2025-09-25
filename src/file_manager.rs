use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct FileManager {
    output_dir: String,
    spine_name: String,
}

impl FileManager {
    pub fn new(output_dir: &str, spine_name: &str) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            spine_name: spine_name.to_string(),
        }
    }

    pub fn create_output_dirs(&self, count: u32) -> Result<()> {
        // 创建输出根目录
        if !Path::new(&self.output_dir).exists() {
            fs::create_dir_all(&self.output_dir)?;
        }

        // 创建每个子目录
        for i in 1..=count {
            let dir_name = format!("{}_{:02}", self.spine_name, i);
            let dir_path = format!("{}/{}", self.output_dir, dir_name);

            if Path::new(&dir_path).exists() {
                fs::remove_dir_all(&dir_path)?;
            }
            fs::create_dir_all(&dir_path)?;
        }

        Ok(())
    }

    pub fn copy_spine_files(
        &self,
        dir_name: &str,
        atlas_path: &str,
        _png_path: &str,
        skel_path: &str,
    ) -> Result<()> {
        let target_dir = format!("{}/{}", self.output_dir, dir_name);

        // 复制atlas文件
        let atlas_target = format!("{}/{}.atlas", target_dir, self.spine_name);
        fs::copy(atlas_path, &atlas_target)?;

        // 复制skel文件
        let skel_target = format!("{}/{}.skel", target_dir, self.spine_name);
        fs::copy(skel_path, &skel_target)?;

        // PNG文件会在图片处理模块中处理，这里不需要复制

        Ok(())
    }
}
