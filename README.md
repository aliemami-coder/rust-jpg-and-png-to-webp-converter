
# Image Conversion Tool
This is a simple command-line tool written in Rust for converting PNG and JPG images to the WebP format. WebP is a modern image format that provides superior compression and quality compared to PNG and JPG, making it suitable for web optimization.

## Prerequisites
Before using this tool, ensure that you have Rust installed on your system. You can download and install Rust from [rust-lang.org](https://rust-lang.org). 

## Installation
Clone this repository to your local machine:
```bash
git clone https://github.com/your-username/image-conversion-tool.git
```

## Usage
Navigate to the directory containing the cloned repository and build the project using Cargo:

```bash
cd image-conversion-tool
cargo build --release
```

After building the project, you can run the executable with the following command:

```bash
./target/release/image-conversion-tool <input_file_or_dir> <output_dir>
```

Replace ```<input_file_or_dir>``` with the path to the PNG or JPG image file you want to convert or the directory containing multiple images. Replace ```<output_dir>``` with the directory where you want to save the converted WebP images.

## Example:
```bash
./target/release/image-conversion-tool input.png output_dir
```
or
```bash
./target/release/image-conversion-tool input_dir/ output_dir
```

## License
This project is licensed under the MIT License - see the ```LICENSE``` file for details.

## Acknowledgments
This tool uses the following libraries:
- [image](https://docs.rs/image) - A library for decoding and encoding various image formats in Rust.

## Contributing
Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## Author

- [@aliemami-coder](https://github.com/aliemami-coder)
