
const FLAG_SIZE: i8 = 64;
const FLAG_STYLE: &'static str = "flat";
const FLAG_API_URL: &'static str = "https://flagsapi.com/";

#[tokio::main]
async fn main() {
    let flag = get_flag("US").await;
    println!("Flag: {:?}", flag);
}

async fn get_flag(country_code: &str) -> Result<String, reqwest::Error> {
    let url = create_flag_url(country_code);
    // let response = reqwest::get(&url).await?;
    // let bytes = response.bytes().await?;
    Ok(url)
}

fn create_flag_url(country_code: &str) -> String {
    format!("{}{}/{}/{}.png", FLAG_API_URL, country_code, FLAG_STYLE, FLAG_SIZE)
}
