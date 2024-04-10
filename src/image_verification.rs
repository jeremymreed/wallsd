use image::io::Reader;
use image::ImageFormat;

pub fn is_supported_format(absolute_path: &String) -> bool {
    let reader = Reader::open(absolute_path)
        .unwrap()
        .with_guessed_format()
        .expect("Failed to open image file");

    let raw_format = match reader.format() {
        Some(format) => format,
        None => {
            // Skip over file.
            tracing::warn!("{}: Unrecognized format", absolute_path);
            return false;
        }
    };

    match raw_format {
        ImageFormat::Png => true,
        ImageFormat::Jpeg => true,
        ImageFormat::Gif => true,
        ImageFormat::WebP => true,
        _ => {
            // Skip over file.
            tracing::warn!("{}: Unsupported format", absolute_path);
            false
        }
    }
}