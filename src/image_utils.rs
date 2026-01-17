use image::{ open, RgbaImage };
use std::path::PathBuf;

// automatically detect constant color and crop it 
// out of the image located at path.
// images which are entirely a solid color will be left untouched
pub fn crop_image(path: &Path) {
    let mut img = open(path).expect("Failed to open image").to_rgba8();
    let (width, height) = img.dimensions();

    let (top, bottom, left, right) = get_crop_bounds(img);
    println!("top: {}, bottom: {}, left: {}, right: {}", top, bottom, left, right);
    let cropped = crop(&mut img, left, top, width - left - right, height - top - bottom).to_image();
    cropped.save().unwrap();
}

// returns (top, bottom, left, right). Each value represents
// how many pixels from the respective side should be cropped
fn get_crop_bounds(img: &RgbaImage) -> (i32, i32, i32, i32) {
    let (width, height) = img.dimensions();
    let (mut top, mut bottom, mut left, mut right) = (0, 0, 0, 0);
    let reference_pixel = img.get_pixel(0, 0).0;

    for y in 0..height {
        let mut valid = true;
        
        for x in 0..width {
            if img.get_pixel(x, y) != reference_pixel {
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }

        top += 1;
    }

    for y in (top..height).rev() {
        let mut valid = true;

        for x in 0..width {
            if img.get_pixel(x, y) != reference_pixel {
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }

        bottom += 1;
    }

    for x in 0..width {
        let mut valid = true;
        
        for y in bottom..top {
            if img.get_pixel(x, y) != reference_pixel {
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }

        left += 1;

    }

    for x in (left..width).rev() {
        let mut valid = true;

        for y in bottom..top {
            if img.get_pixel(x, y) != reference_pixel {
                valid = false;
                break;
            }
        }

        if !valid {
            break;
        }

        right += 1
    }
}
