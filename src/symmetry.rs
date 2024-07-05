use image::{DynamicImage, GenericImageView};

#[derive(Debug)]
pub struct Symmetry {
    pub horizontal: bool,
    pub vertical: bool,
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

pub fn check_symmetry(img: &DynamicImage) -> Symmetry {
    let vertical = check_vertical_symmetry(img);
    let horizontal = check_horizontal_symmetry(img);
    Symmetry {
        horizontal,
        vertical,
    }
}
