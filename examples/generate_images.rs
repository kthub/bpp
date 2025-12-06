use image::{Rgb, RgbImage};
use std::fs;

fn main() {
    fs::create_dir_all("test_images/subdir").unwrap();

    create_image("test_images/test.png", [255, 0, 0]); // Red
    create_image("test_images/test.jpg", [0, 0, 255]); // Blue
    create_image("test_images/test.bmp", [0, 255, 0]); // Green
    create_image("test_images/test.gif", [255, 255, 0]); // Yellow
    create_image("test_images/subdir/subtest.png", [128, 0, 128]); // Purple

    println!("Generated test images in test_images/");
}

fn create_image(path: &str, color: [u8; 3]) {
    let mut img = RgbImage::new(100, 100);
    for x in 0..100 {
        for y in 0..100 {
            img.put_pixel(x, y, Rgb(color));
        }
    }
    img.save(path).unwrap();
    println!("Created {}", path);
}
