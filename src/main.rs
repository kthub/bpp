use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about = "Calculates Bits Per Pixel (BPP) for images")]
struct Cli {
    /// Target directory
    #[arg(default_value = ".")]
    target_dir: PathBuf,

    /// Recursive search
    #[arg(short, long)]
    recursive: bool,

    /// BPP Threshold (only show files with BPP > threshold)
    #[arg(short, long)]
    threshold: Option<f64>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.target_dir.exists() {
        anyhow::bail!("Directory '{}' does not exist.", args.target_dir.display());
    }

    if !args.target_dir.is_dir() {
        anyhow::bail!("'{}' is not a directory.", args.target_dir.display());
    }

    let supported_extensions = ["png", "jpg", "jpeg", "bmp", "gif"];

    let walker = if args.recursive {
        WalkDir::new(&args.target_dir)
    } else {
        WalkDir::new(&args.target_dir).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        process_image(&path.to_path_buf(), args.threshold)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_image(path: &PathBuf, threshold: Option<f64>) -> Result<()> {
    let metadata = std::fs::metadata(path).context("Failed to get file metadata")?;
    let file_size = metadata.len();

    match image::image_dimensions(path) {
        Ok((width, height)) => {
            if width == 0 || height == 0 {
                // Ignore zero dimension images silently or log debug? 
                // Requirement doesn't specify, but better not division by zero.
                return Ok(());
            }

            let bpp = (file_size * 8) as f64 / (width as u64 * height as u64) as f64;

            if let Some(thresh) = threshold {
                if bpp <= thresh {
                    return Ok(());
                }
            }
            
            // Output format: "bpp"<TAB>"path" (no quotes around values)
            // Using full path as requested
            let full_path = std::fs::canonicalize(path).unwrap_or(path.clone());
            println!("{:.2}\t{}", bpp, full_path.display());
        }
        Err(_) => {
            // Silently ignore images that can't be read, or maybe warn?
            // "image_bpp" skipped warning for most parts, let's keep it clean as it's a tool outputting data.
        }
    }

    Ok(())
}
