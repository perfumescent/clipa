use std::borrow::Cow;
use base64::encode;
use image::{ImageBuffer, RgbaImage};

fn image_data_to_base64_jpeg(image_data: Cow<'_, [u8]>, width: usize, height: usize) -> String {
    let width_u32 = u32::try_from(width).expect("width is too large to fit into a u32");
    let height_u32 = u32::try_from(height).expect("height is too large to fit into a u32");

    // 使用 ImageBuffer 来创建一个 RgbaImage
    let img: RgbaImage = ImageBuffer::from_raw(width_u32, height_u32, image_data.to_vec()).unwrap();

    // 将图像编码为 JPEG
    let mut jpeg_bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 100) // 指定 JPEG 质量
        .encode_image(&img)
        .expect("Failed to encode image to JPEG");

    // 将 JPEG 字节编码为 Base64
    let base64_string = encode(&jpeg_bytes);

    // 创建 Data URL
    format!("data:image/jpeg;base64,{}", base64_string)
}

fn image_data_to_base64_jpeg_thumbnail(
    image_data: Cow<'_, [u8]>,
    width: usize,
    height: usize,
) -> String {
    let width_u32 = u32::try_from(width).expect("width is too large to fit into a u32");
    let height_u32 = u32::try_from(height).expect("height is too large to fit into a u32");

    // 计算新的宽度以保持宽高比
    let new_height = 100u32;
    let new_width = (width_u32 * new_height) / height_u32;
    // 使用 ImageBuffer 来创建一个 RgbaImage
    let thumbnail: RgbaImage =
        ImageBuffer::from_raw(new_width, new_height, image_data.to_vec()).unwrap();
    // 将图像编码为低质量的 JPEG
    let mut jpeg_bytes = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 30) // 指定 JPEG 质量为 50
        .encode_image(&thumbnail)
        .expect("Failed to encode image to JPEG");

    // 将 JPEG 字节编码为 Base64
    let base64_string = encode(&jpeg_bytes);

    // 创建 Data URL
    format!("data:image/jpeg;base64,{}", base64_string)
}