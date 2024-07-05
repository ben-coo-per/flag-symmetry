/// A simple app that reports the symmetry of a flag.
/// This only focuses on symmetry along the vertical axis.
use image::{DynamicImage, GenericImageView};
mod fetch_flags;
mod consts {
    pub mod countries;
}
use fetch_flags::get_flag;
use prettytable::{Table, row};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_horizontal: i8 = 0;
    let mut total_vertical: i8 = 0;
    let mut total_full: i8 = 0;
    let mut total_none: i8 = 0;


    // first 10 countries
    for (code, name) in consts::countries::COUNTRIES.iter() {
        let img: DynamicImage = get_flag(code).await?;

        let symmetry = check_symmetry(&img);
        let sym_val;

        if symmetry.horizontal && symmetry.vertical { 
            total_full += 1;
            sym_val = "🪩"
        } else if symmetry.vertical {
            total_vertical += 1;
            sym_val = "↔️"
        } else if symmetry.horizontal { 
            total_horizontal += 1;
            sym_val = "↕️"
        } else { 
            total_none += 1;
            sym_val= "❌"
        }

        println!("{} {}",
            sym_val,
            name
        );
    }


    let mut table = Table::new();
    println!("\nIn summary: ");
    table.add_row(row!["🪩 Flags with full symmetry", total_full]);
    table.add_row(row!["↕️ Flags with horizontal symmetry", total_horizontal]);
    table.add_row(row!["↔️ Flags with vertical symmetry", total_vertical]);
    table.add_row(row!["❌ Flags with no symmetry", total_none]);

    table.printstd();
    Ok(())
}


fn check_vertical_symmetry(img:&DynamicImage) -> bool {
    // symmetrical if the left half is the same as the right half (think "A")
    let (width, height) = img.dimensions();

    // Ensure the image has an even width
    if width % 2 != 0 {
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


fn check_horizontal_symmetry(img:&DynamicImage) -> bool {
    // symmetrical if the top half is the same as the bottom half (think "D")
    let (width, height) = img.dimensions();

    // Ensure the image has an even height
    if height % 2 != 0 {
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
    };
    true
}

#[derive(Debug)]
struct Symmetry {
    horizontal: bool,
    vertical: bool,
}

fn check_symmetry(img:&DynamicImage) -> Symmetry {
    let vertical = check_vertical_symmetry(img);
    let horizontal = check_horizontal_symmetry(img);
    Symmetry { horizontal, vertical }
}