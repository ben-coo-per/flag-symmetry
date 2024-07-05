use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageError, ImageFormat};
use std::io::Cursor;
use bytes::Bytes;

const FLAG_SIZE: &'static str = "w20";
const FLAG_API_URL: &'static str = "https://flagcdn.com";

fn load_image_from_bytes(bytes: &Bytes) -> Result<DynamicImage, ImageError > {
    let cursor = Cursor::new(bytes);
    let mut reader = ImageReader::new(cursor);
    reader.set_format(ImageFormat::Png);
    let img = reader.decode()?;
    Ok(img)
}

fn create_flag_url(country_code: &str) -> String {
    format!("{}/{}/{}.png", FLAG_API_URL, FLAG_SIZE, country_code)
}

pub async fn get_flag(country_code: &str) -> Result<DynamicImage, reqwest::Error> {
    let url = create_flag_url(country_code);
    let response = reqwest::get(&url).await?;

    if !response.status().is_success() {
        panic!("Failed to fetch the flag for country code {}. Status code: {}", country_code, response.status());
    }

    let bytes = response.bytes().await?;
    let img: DynamicImage = load_image_from_bytes(&bytes).expect("Failed to load image from bytes");
    Ok(img)
}