#[napi]
struct ImageImpl {}

#[napi]
impl ImageImpl {
    #[napi]
    pub fn ppm_file2png_file(ppm_path: String, png_path: String) -> bool {
        let img = image::open(&std::path::Path::new(&ppm_path)).unwrap();

        match img.save_with_format(&png_path, image::ImageFormat::Png) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn ppm_file2png_buffer(ppm_path: String) -> napi::Result<Vec<u8>> {
        let img = image::open(&std::path::Path::new(&ppm_path)).unwrap();

        // 将 img 转换为 png 并返回 buffer
        let mut buffer = vec![];
        img.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
    }
}