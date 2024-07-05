/// A simple app that reports the symmetry of a flag.
/// This only focuses on symmetry along the vertical axis.
use image::{DynamicImage, GenericImageView};
mod fetch_flags;
mod consts {
    pub mod countries;
}
use fetch_flags::get_flag;
use prettytable::{row, Table};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_horizontal: (i64, String) = (0, "".to_string());
    let mut total_vertical: (i64, String) = (0, "".to_string());
    let mut total_full: (i64, String) = (0, "".to_string());
    let mut total_none: (i64, String) = (0, "".to_string());

    // Loop through all countries, but chunk into groups of 10 to avoid hitting rate limits
    let countries = consts::countries::COUNTRIES
        .iter()
        .map(|(code, (name, emoji))| (code.to_string(), (name.to_string(), emoji.to_string())))
        .collect::<Vec<(String, (String, String))>>();
    let chunked_countries = countries
        .chunks(120)
        .into_iter()
        .collect::<Vec<&[(String, (String, String))]>>();
    for chunk in chunked_countries {
        // Sleep for a bit to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        for (code, name) in chunk {
            let img: DynamicImage = get_flag(code).await?;

            let symmetry = check_symmetry(&img);
            let sym_val;

            if symmetry.horizontal && symmetry.vertical {
                update_table_total_value(&mut total_full, name);
                sym_val = "🪩";
            } else if symmetry.vertical {
                update_table_total_value(&mut total_vertical, name);
                sym_val = "↔️";
            } else if symmetry.horizontal {
                update_table_total_value(&mut total_horizontal, name);
                sym_val = "↕️";
            } else {
                update_table_total_value(&mut total_none, name);
                sym_val = "❌";
            }

            println!("{} {}{}", sym_val, name.0, name.1);
        }
    }

    let mut table = Table::new();

    println!("\nIn summary: ");
    table.add_row(row!["🪩 Flags with full symmetry", total_full.0]);
    table.add_row(row!["↕️ Flags with horizontal symmetry", total_horizontal.0]);
    table.add_row(row!["↔️ Flags with vertical symmetry", total_vertical.0]);
    table.add_row(row!["❌ Flags with no symmetry", total_none.0]);

    table.printstd();

    // delay for a sec
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    println!("All full symmetry flags: {}", total_full.1);
    println!("All horizontal symmetry flags: {}", total_horizontal.1);
    println!("All vertical symmetry flags: {}", total_vertical.1);
    println!("All no symmetry flags: {}", total_none.1);
    Ok(())
}

fn update_table_total_value(table_total: &mut (i64, String), name: &(String, String)) {
    table_total.0 += 1;
    table_total.1.push_str(name.1.as_str());
}

fn check_vertical_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the left half is the same as the right half (think "A")
    let (width, height) = img.dimensions();

    // Ensure the image has an even width
    if width % 2 != 0 {
        println!("width is not even: {}", width);
        return false;
    }

    let half_width = width / 2;
    for y in 0..height {
        for x in 0..half_width {
            let left_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(width - 1 - x, y);
            if left_pixel != right_pixel {
                return false;
            }
        }
    }
    true
}

fn check_horizontal_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the top half is the same as the bottom half (think "D")
    let (width, height) = img.dimensions();

    // Ensure the image has an even height
    if height % 2 != 0 {
        println!("height is not even: {}", height);
        return false;
    }

    let half_height = height / 2;
    for x in 0..width {
        for y in 0..half_height {
            let top_pixel = img.get_pixel(x, y);
            let bottom_pixel = img.get_pixel(x, height - 1 - y);
            if top_pixel != bottom_pixel {
                return false;
            }
        }
    }
    true
}

#[derive(Debug)]
struct Symmetry {
    horizontal: bool,
    vertical: bool,
}

fn check_symmetry(img: &DynamicImage) -> Symmetry {
    let vertical = check_vertical_symmetry(img);
    let horizontal = check_horizontal_symmetry(img);
    Symmetry {
        horizontal,
        vertical,
    }
}
