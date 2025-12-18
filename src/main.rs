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

    /// Sort by BPP descending
    #[arg(short, long)]
    sort: bool,
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

    let mut results: Vec<(f64, String)> = Vec::new();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if supported_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        if let Ok(Some(bpp)) = calculate_bpp(&path.to_path_buf()) {
                            if let Some(thresh) = args.threshold {
                                if bpp <= thresh {
                                    continue;
                                }
                            }

                            // Get absolute path string
                            let full_path = std::fs::canonicalize(path)
                                .unwrap_or(path.to_path_buf())
                                .display()
                                .to_string();

                            if args.sort {
                                results.push((bpp, full_path));
                            } else {
                                println!("{:.2}\t{}", bpp, full_path);
                            }
                        }
                    }
                }
            }
        }
    }

    if args.sort {
        // Sort by BPP descending
        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        for (bpp, path) in results {
            println!("{:.2}\t{}", bpp, path);
        }
    }

    Ok(())
}

fn calculate_bpp(path: &PathBuf) -> Result<Option<f64>> {
    let metadata = std::fs::metadata(path).context("Failed to get file metadata")?;
    let file_size = metadata.len();

    match image::image_dimensions(path) {
        Ok((width, height)) => {
            if width == 0 || height == 0 {
                return Ok(None);
            }

            let bpp = (file_size * 8) as f64 / (width as u64 * height as u64) as f64;
            Ok(Some(bpp))
        }
        Err(_) => {
            // Silently ignore images that can't be read
            Ok(None)
        }
    }
}
