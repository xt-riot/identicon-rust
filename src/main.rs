use image::{ImageBuffer, Rgb, RgbImage};
use sha3::{Digest, Sha3_512};
fn main() {
    let input = "konstantinos.karachristos";
    let mut hasher = Sha3_512::new();
    hasher.update(input.as_bytes());

    let hashed_input = &hasher.finalize();

    let identicon: Box<dyn Identicon<u8>> = Box::new(Implementation);

    let img = identicon.get_identicon(hashed_input);

    img.save("output.jpg").unwrap();
}

trait Identicon<T> {
    fn get_color(&self, input: &[T]) -> Color<T>;
    fn get_image(&self, input: &[T], color: Color<T>) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn get_identicon(&self, input: &[T]) -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}

#[derive(Debug)]
struct Color<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

struct Implementation;

impl Identicon<u8> for Implementation {
    fn get_color(&self, input: &[u8]) -> Color<u8> {
        Color {
            red: input[0],
            green: input[1],
            blue: input[2],
        }
    }

    fn get_image(&self, input: &[u8], color: Color<u8>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = RgbImage::new(10, 10);
        let mut counter = 0;

        for x in 0..5 {
            for y in 0..10 {
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

    fn get_identicon(&self, input: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let color = self.get_color(input);
        self.get_image(input, color)
    }
}
