use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageError};
use std::io::Cursor;
use bytes::Bytes;

const FLAG_SIZE: i8 = 64;
const FLAG_STYLE: &'static str = "flat";
const FLAG_API_URL: &'static str = "https://flagsapi.com/";

fn load_image_from_bytes(bytes: &Bytes) -> Result<DynamicImage, ImageError > {
    let cursor = Cursor::new(bytes);
    let img = ImageReader::new(cursor)
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}

fn create_flag_url(country_code: &str) -> String {
    format!("{}{}/{}/{}.png", FLAG_API_URL, country_code, FLAG_STYLE, FLAG_SIZE)
}

pub async fn get_flag(country_code: &str) -> Result<DynamicImage, reqwest::Error> {
    let url = create_flag_url(country_code);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    let img: DynamicImage = load_image_from_bytes(&bytes).expect("Failed to load image from bytes");
    Ok(img)
}