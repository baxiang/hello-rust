## 第六步：滤镜效果

### src/processor/filter.rs

```rust
use image::{DynamicImage, ImageBuffer, Rgb, Rgba};

/// 滤镜类型
pub enum Filter {
    /// 灰度
    Grayscale,
    /// 反色
    Invert,
    /// 亮度调整 (-100 到 100)
    Brightness(i32),
    /// 对比度调整 (-100 到 100)
    Contrast(i32),
    /// 模糊
    Blur(f32),
    /// 锐化
    Sharpen,
}

/// 应用滤镜
pub fn apply_filter(image: &DynamicImage, filter: &Filter) -> DynamicImage {
    match filter {
        Filter::Grayscale => grayscale(image),
        Filter::Invert => invert(image),
        Filter::Brightness(amount) => brightness(image, *amount),
        Filter::Contrast(amount) => contrast(image, *amount),
        Filter::Blur(sigma) => image.blur(*sigma),
        Filter::Sharpen => image.unsharpen(1.0, 5),
    }
}

/// 灰度处理
fn grayscale(image: &DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(image.to_luma8())
}

/// 反色处理
fn invert(image: &DynamicImage) -> DynamicImage {
    let mut img = image.clone();
    img.invert();
    img
}

/// 亮度调整
fn brightness(image: &DynamicImage, amount: i32) -> DynamicImage {
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    
    let adjusted: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(
        width, height,
        |x, y| {
            let pixel = rgb_image.get_pixel(x, y);
            
            let adjust = |v: u8| -> u8 {
                let new_val = v as i32 + amount;
                new_val.clamp(0, 255) as u8
            };
            
            Rgb([adjust(pixel[0]), adjust(pixel[1]), adjust(pixel[2])])
        }
    );
    
    DynamicImage::ImageRgb8(adjusted)
}

/// 对比度调整
fn contrast(image: &DynamicImage, amount: i32) -> DynamicImage {
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    
    let factor = (259.0 * (amount as f32 + 255.0)) / (255.0 * (259.0 - amount as f32));
    
    let adjusted: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(
        width, height,
        |x, y| {
            let pixel = rgb_image.get_pixel(x, y);
            
            let adjust = |v: u8| -> u8 {
                let new_val = factor * (v as f32 - 128.0) + 128.0;
                new_val.clamp(0.0, 255.0) as u8
            };
            
            Rgb([adjust(pixel[0]), adjust(pixel[1]), adjust(pixel[2])])
        }
    );
    
    DynamicImage::ImageRgb8(adjusted)
}
```






## 第七步：批量处理

### src/batch/worker.rs

```rust
use image::DynamicImage;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::processor::{resize, convert, compress, filter};
use crate::utils::format::ImageFormat;

/// 批量处理选项
pub struct BatchOptions {
    /// 输入目录
    pub input_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 输出格式
    pub output_format: Option<ImageFormat>,
    /// 调整大小
    pub resize: Option<resize::ResizeOptions>,
    /// 压缩质量
    pub quality: Option<u8>,
    /// 滤镜
    pub filters: Vec<filter::Filter>,
}

/// 批量处理
pub fn process_batch(options: BatchOptions) -> anyhow::Result<()> {
    // 创建输出目录
    std::fs::create_dir_all(&options.output_dir)?;
    
    // 收集所有图片文件
    let files: Vec<PathBuf> = WalkDir::new(&options.input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| ImageFormat::from_path(e.path()).is_some())
        .map(|e| e.path().to_path_buf())
        .collect();
    
    if files.is_empty() {
        println!("未找到图片文件");
        return Ok(());
    }
    
    println!("找到 {} 个图片文件", files.len());
    
    // 创建进度条
    let progress = ProgressBar::new(files.len() as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{pos}/{len} [{bar:40}] {msg}")
            .unwrap()
    );
    
    // 并行处理
    files.par_iter().for_each(|file| {
        let result = process_single(file, &options);
        
        match result {
            Ok(output_path) => {
                progress.inc(1);
                progress.set_message(format!("已处理: {}", file.display()));
            }
            Err(e) => {
                progress.inc(1);
                progress.set_message(format!("错误: {} - {}", file.display(), e));
            }
        }
    });
    
    progress.finish_with_message("处理完成");
    
    Ok(())
}

/// 处理单个文件
fn process_single(input_path: &Path, options: &BatchOptions) -> anyhow::Result<PathBuf> {
    // 加载图片
    let img = image::open(input_path)?;
    let mut processed = img.clone();
    
    // 应用滤镜
    for f in &options.filters {
        processed = filter::apply_filter(&processed, f);
    }
    
    // 调整大小
    if let Some(resize_opts) = &options.resize {
        processed = resize::resize(&processed, resize_opts);
    }
    
    // 确定输出格式
    let output_format = options.output_format
        .unwrap_or_else(|| {
            ImageFormat::from_path(input_path).unwrap_or(ImageFormat::Jpeg)
        });
    
    // 生成输出路径
    let output_filename = input_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + "."
        + output_format.extension();
    
    let output_path = options.output_dir.join(output_filename);
    
    // 保存
    processed.save_with_format(&output_path, output_format.to_image_format())?;
    
    Ok(output_path)
}
```






## 第八步：主入口

### src/main.rs

```rust
use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;

mod utils;
mod processor;
mod batch;

#[derive(Parser)]
#[command(name = "image-processor")]
#[command(about = "图片处理工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 调整图片大小
    Resize {
        /// 输入文件
        #[arg(short, long)]
        input: PathBuf,
        
        /// 输出文件
        #[arg(short, long)]
        output: PathBuf,
        
        /// 宽度
        #[arg(short = 'W', long)]
        width: Option<u32>,
        
        /// 高度
        #[arg(short = 'H', long)]
        height: Option<u32>,
    },
    
    /// 转换格式
    Convert {
        /// 输入文件
        #[arg(short, long)]
        input: PathBuf,
        
        /// 输出文件
        #[arg(short, long)]
        output: PathBuf,
        
        /// 输出格式
        #[arg(short, long)]
        format: String,
    },
    
    /// 批量处理
    Batch {
        /// 输入目录
        #[arg(short, long)]
        input: PathBuf,
        
        /// 输出目录
        #[arg(short, long)]
        output: PathBuf,
        
        /// 输出格式
        #[arg(short, long)]
        format: Option<String>,
        
        /// 宽度
        #[arg(short = 'W', long)]
        width: Option<u32>,
        
        /// 高度
        #[arg(short = 'H', long)]
        height: Option<u32>,
        
        /// 质量 (1-100)
        #[arg(short = 'q', long)]
        quality: Option<u8>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Resize { input, output, width, height } => {
            let img = image::open(&input)?;
            
            let options = processor::resize::ResizeOptions::new();
            let options = if let Some(w) = width {
                options.width(w)
            } else {
                options
            };
            let options = if let Some(h) = height {
                options.height(h)
            } else {
                options
            };
            
            let resized = processor::resize::resize(&img, &options);
            resized.save(&output)?;
            
            println!("已保存: {}", output.display());
        }
        
        Commands::Convert { input, output, format } => {
            let img = image::open(&input)?;
            let output_format = utils::format::ImageFormat::from_extension(&format)
                .ok_or_else(|| anyhow::anyhow!("不支持的格式: {}", format))?;
            
            img.save_with_format(&output, output_format.to_image_format())?;
            
            println!("已转换: {}", output.display());
        }
        
        Commands::Batch { input, output, format, width, height, quality } => {
            let output_format = format.and_then(|f| utils::format::ImageFormat::from_extension(&f));
            
            let resize_opts = if width.is_some() || height.is_some() {
                let mut opts = processor::resize::ResizeOptions::new();
                if let Some(w) = width {
                    opts = opts.width(w);
                }
                if let Some(h) = height {
                    opts = opts.height(h);
                }
                Some(opts)
            } else {
                None
            };
            
            let options = batch::worker::BatchOptions {
                input_dir: input,
                output_dir: output,
                output_format,
                resize: resize_opts,
                quality,
                filters: vec![],
            };
            
            batch::worker::process_batch(options)?;
        }
    }
    
    Ok(())
}
```






## 运行效果

```bash
# 调整大小
image-processor resize -i input.jpg -o output.jpg -W 800 -H 600

# 格式转换
image-processor convert -i input.png -o output.jpg --format jpg

# 批量处理
image-processor batch -i ./images -o ./output --format jpg -W 800 -q 85
```

### 输出示例

```
找到 100 个图片文件
100/100 [████████████████████████████████] 已处理: ./images/photo99.jpg
处理完成
```






## 小结

本项目涵盖：

- ✅ 图片读写操作
- ✅ 格式转换
- ✅ 大小调整
- ✅ 滤镜效果
- ✅ 批量处理
- ✅ 并行加速







