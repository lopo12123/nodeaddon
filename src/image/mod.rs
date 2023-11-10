use std::io::Cursor;
use image::ImageResult;

#[napi]
struct ImageImpl {}

#[napi]
impl ImageImpl {
    /// 将 ppm 文件转换为 png 文件并写入到指定路径, 成功则返回 true
    #[napi]
    pub fn ppm_file2png_file(ppm_path: String, png_path: String) -> bool {
        let img = image::open(&std::path::Path::new(&ppm_path)).unwrap();

        match img.save_with_format(&png_path, image::ImageFormat::Png) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// 将 ppm 文件转换为 png buffer, 失败则返回 None
    #[napi]
    pub fn ppm_file2png_buffer(ppm_path: String) -> Option<Vec<u8>> {
        let img = image::open(&std::path::Path::new(&ppm_path)).unwrap();

        // 将 img 转换为 png 并返回 buffer
        let mut buffer = Cursor::new(vec![]);
        match img.write_to(&mut buffer, image::ImageFormat::Png) {
            Ok(_) => Some(buffer.into_inner()),
            Err(_) => None,
        }
    }

    /// 将 ppm buffer 转换为 png buffer, 失败则返回 None
    pub fn ppm_buffer2png_buffer(ppm_buffer: Vec<u8>) -> Option<Vec<u8>> {
        // TODO
        None
    }
}