use image::{DynamicImage, GenericImageView, Rgba};

const FUZZY_WINDOW_DIM: usize = 3; // size of the one axis of window to use for fuzzy comparison
const FUZZY_WINDOW_SIZE: usize = usize::pow(FUZZY_WINDOW_DIM, 2);
const COLOR_TOLERANCE: f32 = 0.1; // percentage tolerance for color comparison

#[derive(Debug)]
pub struct Symmetry {
    pub horizontal: bool,
    pub vertical: bool,
}

pub fn check_symmetry(img: &DynamicImage) -> Symmetry {
    let vertical = check_vertical_symmetry(img);
    let horizontal = check_horizontal_symmetry(img);
    Symmetry {
        horizontal,
        vertical,
    }
}

fn check_vertical_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the left half is the same as the right half (think "A")
    let (width, height) = img.dimensions();

    // note: we can ignore edge cases where image is not even since the u32 rounds down and we'll ignore the center pixel in each row
    let half_width = width / 2;
    let bump: u32 = if width % 2 == 0 { 0 } else { 1 }; // bump the window by 1 if it's odd since u32 rounds down

    let left_window = crop_image(&img, 0, 0, half_width, height);
    let right_window = crop_image(&img, half_width + bump, 0, half_width, height).fliph(); // flip the right window to match the left

    for y in 0..height {
        for x in 0..half_width {
            let left_pixel = left_window.get_pixel(x, y);
            let right_pixel = right_window.get_pixel(x, y);

            if !compare_pixel(left_pixel, right_pixel) {
                if check_near_edge(x, y, half_width, height) {
                    continue;
                }
                let small_left = crop_image(
                    &left_window,
                    x - (FUZZY_WINDOW_DIM as u32) / 2,
                    y - (FUZZY_WINDOW_DIM as u32) / 2,
                    FUZZY_WINDOW_DIM as u32,
                    FUZZY_WINDOW_DIM as u32,
                );

                let small_right = crop_image(
                    &right_window,
                    x - (FUZZY_WINDOW_DIM as u32) / 2,
                    y - (FUZZY_WINDOW_DIM as u32) / 2,
                    FUZZY_WINDOW_DIM as u32,
                    FUZZY_WINDOW_DIM as u32,
                );

                if !fuzzy_compare(small_left, small_right) {
                    return false;
                }
            }
        }
    }
    true
}

fn check_horizontal_symmetry(img: &DynamicImage) -> bool {
    // symmetrical if the top half is the same as the bottom half (think "D")
    let (width, height) = img.dimensions();

    // note: we can ignore edge cases where image is not even since the u32 rounds down and we'll ignore the center pixel in each column
    let half_height = height / 2;
    let bump: u32 = if height % 2 == 0 { 0 } else { 1 }; // bump the window by 1 if it's odd since u32 rounds down

    let top_window = crop_image(&img, 0, 0, width, half_height);
    let bottom_window = crop_image(&img, 0, half_height + bump, width, half_height).flipv(); // flip the bottom window to match the top

    for x in 0..width {
        for y in 0..half_height {
            let top_pixel = top_window.get_pixel(x, y);
            let bottom_pixel = bottom_window.get_pixel(x, y);

            if !compare_pixel(top_pixel, bottom_pixel) {
                if check_near_edge(x, y, width, half_height) {
                    continue;
                }

                let small_top = crop_image(
                    &top_window,
                    x - (FUZZY_WINDOW_DIM as u32) / 2,
                    y - (FUZZY_WINDOW_DIM as u32) / 2,
                    FUZZY_WINDOW_DIM as u32,
                    FUZZY_WINDOW_DIM as u32,
                );

                let small_bottom = crop_image(
                    &bottom_window,
                    x - (FUZZY_WINDOW_DIM as u32) / 2,
                    y - (FUZZY_WINDOW_DIM as u32) / 2,
                    FUZZY_WINDOW_DIM as u32,
                    FUZZY_WINDOW_DIM as u32,
                );

                if !fuzzy_compare(small_top, small_bottom) {
                    return false;
                }
            }
        }
    }
    true
}

fn compare_pixel(a: Rgba<u8>, b: Rgba<u8>) -> bool {
    // Compare two pixels, returning true if they are within a certain tolerance of each other.
    // Sum up the values of the RGB channels and compare them. If they are within the tolerance % of each other, return true.
    let [r1, g1, b1, _a1] = a.0;
    let [r2, g2, b2, _a2] = b.0;

    color_channel_compare(r1 as i16, r2 as i16)
        && color_channel_compare(g1 as i16, g2 as i16)
        && color_channel_compare(b1 as i16, b2 as i16)
}

fn color_channel_compare(c1: i16, c2: i16) -> bool {
    // Compare two color channels, returning true if they are within a certain tolerance of each other.
    // Calculate the average value to compare against
    let average_value = ((c1 + c2) / 2) as f32;

    // Avoid division by zero by checking if the average_value is zero
    if average_value == 0.0 {
        return c1 == c2;
    }

    let diff = (c1 - c2).abs() as f32;

    (diff / average_value) < COLOR_TOLERANCE
}

fn fuzzy_compare(a: DynamicImage, b: DynamicImage) -> bool {
    // Compare two cropped images, returning true if they are within a certain tolerance of each other.
    let mut avg_a: [f32; 3] = [0.0, 0.0, 0.0];
    let mut avg_b: [f32; 3] = [0.0, 0.0, 0.0];
    let mut diff: [f32; 3] = [0.0, 0.0, 0.0];

    let (width, height) = a.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel_a = a.get_pixel(x, y);
            let pixel_b = b.get_pixel(x, y);

            for i in 0..3 {
                avg_a[i] += pixel_a.0[i] as f32;
                avg_b[i] += pixel_b.0[i] as f32;
            }
        }
    }

    // divide each channel in avg_a & avg_b by the number of pixels to get the average
    for i in 0..3 {
        avg_a[i] /= FUZZY_WINDOW_SIZE as f32;
        avg_b[i] /= FUZZY_WINDOW_SIZE as f32;
        diff[i] = (avg_a[i] - avg_b[i]).abs();
    }

    for i in 0..3 {
        let avg = (avg_a[i] + avg_b[i]) / 2.0;
        if avg != 0.0 && (diff[i] / avg) > COLOR_TOLERANCE {
            return false;
        }
    }

    true
}

fn crop_image(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    let view = img.view(x, y, width, height).to_image();
    DynamicImage::ImageRgba8(view)
}

fn check_near_edge(x: u32, y: u32, width: u32, height: u32) -> bool {
    let dis_to_edge = FUZZY_WINDOW_DIM as i16 / 2;
    (y as i16) - dis_to_edge / 2 <= 0
        || (y as i16) + dis_to_edge >= height as i16
        || (x as i16) - dis_to_edge <= 0
        || (x as i16) + dis_to_edge >= width as i16
}
