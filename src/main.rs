/// A simple app that reports the symmetry of a flag.
/// This only focuses on symmetry along the vertical axis.
use image::DynamicImage;
mod fetch_flags;
mod symmetry;
mod consts {
    pub mod countries;
}

use fetch_flags::get_flag;
use prettytable::{row, Table};
use symmetry::check_symmetry;

const CHUNK_SIZE: usize = 120;
const TIME_DELAY: u64 = 1;

#[derive(Debug)]
struct ReportingValue(i64, String);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut total_horizontal = ReportingValue(0, String::new());
    let mut total_vertical = ReportingValue(0, String::new());
    let mut total_full = ReportingValue(0, String::new());
    let mut total_none = ReportingValue(0, String::new());

    // Loop through all countries, but chunk into groups to avoid hitting rate limits
    let countries = consts::countries::COUNTRIES
        .iter()
        .map(|(code, (name, emoji))| (code.to_string(), (name.to_string(), emoji.to_string())))
        .collect::<Vec<(String, (String, String))>>();

    let chunked_countries = countries
        .chunks(CHUNK_SIZE)
        .into_iter()
        .collect::<Vec<&[(String, (String, String))]>>();

    for chunk in chunked_countries {
        // Sleep for a bit to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_secs(TIME_DELAY)).await;
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

    report_results(total_full, total_horizontal, total_vertical, total_none).await;
    Ok(())
}

fn update_table_total_value(table_total: &mut ReportingValue, name: &(String, String)) {
    table_total.0 += 1;
    table_total.1.push_str(name.1.as_str());
}

async fn report_results(
    total_full: ReportingValue,
    total_horizontal: ReportingValue,
    total_vertical: ReportingValue,
    total_none: ReportingValue,
) {
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
}
