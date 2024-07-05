use image::{DynamicImage, GenericImageView, Rgba};
const COLOR_TOLERANCE: f32 = 0.25;

#[derive(Debug)]
pub struct Symmetry {
    pub horizontal: bool,
    pub vertical: bool,
}

fn check_vertical_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the left half is the same as the right half (think "A")
    let (width, height) = img.dimensions();

    // note: we can ignore edge cases where image is not even since the u32 rounds down and we'll ignore the center pixel in each row
    let half_width = width / 2;
    for y in 0..height {
        for x in 0..half_width {
            let left_pixel = img.get_pixel(x, y);
            let right_pixel = img.get_pixel(width - 1 - x, y);
            if !compare_pixel(left_pixel, right_pixel) {
                return false;
            }
        }
    }
    true
}

fn check_horizontal_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the top half is the same as the bottom half (think "D")
    let (width, height) = img.dimensions();

    // note: height will always be even since we are specifying the height of the image
    let half_height = height / 2;
    for x in 0..width {
        for y in 0..half_height {
            let top_pixel = img.get_pixel(x, y);
            let bottom_pixel = img.get_pixel(x, height - 1 - y);
            if !compare_pixel(top_pixel, bottom_pixel) {
                return false;
            }
        }
    }
    true
}

pub fn check_symmetry(img: &DynamicImage) -> Symmetry {
    let vertical = check_vertical_symmetry(img);
    let horizontal = check_horizontal_symmetry(img);
    Symmetry {
        horizontal,
        vertical,
    }
}

fn compare_pixel(a: Rgba<u8>, b: Rgba<u8>) -> bool {
    // Compare two pixels, returning true if they are within a certain tolerance of each other.
    // Sum up the values of the RGB channels and compare them. If they are within the tolerance % of each other, return true.
    let [r1, g1, b1, _a1] = a.0;
    let [r2, g2, b2, _a2] = b.0;

    let a_value = r1 as f32 + g1 as f32 + b1 as f32;
    let b_value = r2 as f32 + g2 as f32 + b2 as f32;

    // Calculate the average value to compare against
    let average_value = (a_value + b_value) / 2.0;

    // Avoid division by zero by checking if the average_value is zero
    if average_value == 0.0 {
        return a_value == b_value;
    }

    let diff = (a_value - b_value).abs();

    (diff / average_value) < COLOR_TOLERANCE
}
