use arboard::ImageData;
use base64::encode;
use image::{ImageBuffer, RgbaImage};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClipboardImage {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<u8>,
}

impl ClipboardImage {
    pub(crate) fn to_image_data(&self) -> ImageData {
        ImageData {
            width: self.width,
            height: self.height,
            bytes: Cow::Borrowed(&self.bytes),
        }
    }

    pub(crate) fn to_base64_jpeg_thumbnail(&self) -> String {
        let width_u32 = u32::try_from(self.width).expect("width is too large to fit into a u32");
        let height_u32 = u32::try_from(self.height).expect("height is too large to fit into a u32");

        // 如果height_u32高度大于500，则计算新的宽度以保持宽高比；否则，使用原始宽度
        let new_height = if height_u32 > 500 { 500 } else { height_u32 };
        let new_width = (width_u32 * new_height) / height_u32;
        // 使用 ImageBuffer 来创建一个 RgbaImage

        ImageBuffer::from_raw(new_width, new_height, self.bytes.clone()).map_or(
            "Failed to parse image".to_string(),
            |thumbnail: RgbaImage| {
                // 将图像编码为低质量的 JPEG
                let mut jpeg_bytes = Vec::new();
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 80) // 指定 JPEG 质量为 50
                    .encode_image(&thumbnail)
                    .expect("Failed to encode image to JPEG");

                // 将 JPEG 字节编码为 Base64
                let base64_string = encode(&jpeg_bytes);

                // 创建 Data URL
                format!("data:image/jpeg;base64,{}", base64_string)
            },
        )
    }

    pub(crate) fn to_base64_jpeg(self) -> String {
        let width_u32 = u32::try_from(self.width).expect("width is too large to fit into a u32");
        let height_u32 = u32::try_from(self.height).expect("height is too large to fit into a u32");

        // 使用 ImageBuffer 来创建一个 RgbaImage
        ImageBuffer::from_raw(width_u32, height_u32, self.bytes).map_or(
            "Failed to parse image".to_string(),
            |img: RgbaImage| {
                // 将图像编码为 JPEG
                let mut jpeg_bytes = Vec::new();
                image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 100) // 指定 JPEG 质量
                    .encode_image(&img)
                    .expect("Failed to encode image to JPEG");

                // 将 JPEG 字节编码为 Base64
                let base64_string = encode(&jpeg_bytes);

                // 创建 Data URL
                format!("data:image/jpeg;base64,{}", base64_string)
            },
        )
    }
}

impl From<ImageData<'static>> for ClipboardImage {
    fn from(image_data: ImageData<'static>) -> Self {
        Self {
            width: image_data.width,
            height: image_data.height,
            bytes: image_data.bytes.into_owned(),
        }
    }
}
