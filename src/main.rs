use std::env;
use std::fs;
use std::path::PathBuf;
use image::{ImageOutputFormat, ImageFormat};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file_or_dir> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_dir = PathBuf::from(&args[2]);

    if !output_dir.is_dir() {
        eprintln!("Invalid output directory: {}", output_dir.display());
        std::process::exit(1);
    }

    if input_path.is_file() {
        convert_single_image(&input_path, &output_dir);
    } else if input_path.is_dir() {
        convert_images(&input_path, &output_dir);
    } else {
        eprintln!("Invalid input path: {}", input_path.display());
        std::process::exit(1);
    }
}

fn convert_single_image(input_path: &PathBuf, output_dir: &PathBuf) {
    if let Some(ext) = input_path.extension().and_then(|s| s.to_str()) {
        if ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("jpg") {
            let img = image::open(&input_path).unwrap();
            let output_path = generate_output_path(output_dir, input_path, "webp");
            let output_file = std::fs::File::create(&output_path).unwrap();
            img.write_to(&mut std::io::BufWriter::new(output_file), ImageOutputFormat::WebP).unwrap();
            println!("Converted {} to {}", input_path.display(), output_path.display());
        }
    }
}

fn convert_images(input_dir: &PathBuf, output_dir: &PathBuf) {
    let entries = fs::read_dir(input_dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext.eq_ignore_ascii_case("png") || ext.eq_ignore_ascii_case("jpg") {
                    let img = image::open(&path).unwrap();
                    let output_path = generate_output_path(output_dir, &path, "webp");
                    let output_file = std::fs::File::create(&output_path).unwrap();
                    img.write_to(&mut std::io::BufWriter::new(output_file), ImageOutputFormat::WebP).unwrap();
                    println!("Converted {} to {}", path.display(), output_path.display());
                }
            }
        }
    }
}

fn generate_output_path(output_dir: &PathBuf, input_path: &PathBuf, ext: &str) -> PathBuf {
    let input_file_name = input_path.file_name().unwrap().to_str().unwrap();
    let mut output_path = output_dir.join(input_file_name);
    output_path.set_extension(ext);
    output_path
}