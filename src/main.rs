use image::{ImageBuffer, Rgb, RgbImage};
use sha3::{Digest, Sha3_512};
fn main() {
    let input = "";
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());

    let hashed_input = &hasher.finalize();

    let color = get_color(hashed_input);

    let img = get_image(hashed_input, color);
    img.save("output.json").unwrap();
}

fn get_image(input: &[u8], color: Color) -> RgbImage {
    let mut img = RgbImage::new(10, 10);
    let mut counter = 0;

    for x in 0..5 {
        for y in 0..10 {
            img.save(format!("images/output{}{}.jpg", x, y)).unwrap();
            if input[counter] % 2 == 0 {
                img.put_pixel(x, y, Rgb([color.red, color.green, color.blue]));
                img.put_pixel(9 - x, y, Rgb([color.red, color.green, color.blue]));
            } else {
                img.put_pixel(x, y, Rgb([0, 0, 0]));
                img.put_pixel(9 - x, y, Rgb([0, 0, 0]));
            }
            counter += 1;
        }
    }

    img
}

fn get_color(input: &[u8]) -> Color {
    Color {
        red: input[0],
        green: input[1],
        blue: input[2],
    }
}

#[derive(Debug)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
