use wasm_bindgen::prelude::*;
use heic::{DecoderConfig, PixelLayout};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

#[wasm_bindgen]
pub fn convert_heic_to_jpg(data: &[u8], quality: u8) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();

    // Decode HEIC
    let decoded = DecoderConfig::new()
        .decode(data, PixelLayout::Rgba8)
        .map_err(|e| JsValue::from_str(&format!("HEIC decode error: {:?}", e)))?;

    // Create DynamicImage from raw RGBA data
    let img = DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(decoded.width, decoded.height, decoded.data)
            .ok_or_else(|| JsValue::from_str("Failed to create image from raw data"))?
    );

    // Encode to JPG
    let mut jpg_data = Vec::new();
    let mut cursor = Cursor::new(&mut jpg_data);
    
    // We can use img.write_to but to control quality we might need a specific encoder
    // However, image crate's default JPG encoding is usually fine.
    // For specific quality:
    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality);
    encoder.encode_image(&img)
        .map_err(|e| JsValue::from_str(&format!("JPG encode error: {:?}", e)))?;

    Ok(jpg_data)
}

#[wasm_bindgen]
pub fn convert_heic_to_png(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    console_error_panic_hook::set_once();

    // Decode HEIC
    let decoded = DecoderConfig::new()
        .decode(data, PixelLayout::Rgba8)
        .map_err(|e| JsValue::from_str(&format!("HEIC decode error: {:?}", e)))?;

    // Create DynamicImage from raw RGBA data
    let img = DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(decoded.width, decoded.height, decoded.data)
            .ok_or_else(|| JsValue::from_str("Failed to create image from raw data"))?
    );

    // Encode to PNG
    let mut png_data = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_data), ImageFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("PNG encode error: {:?}", e)))?;

    Ok(png_data)
}
