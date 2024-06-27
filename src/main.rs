use base64::{engine::general_purpose::URL_SAFE, Engine as _};

fn main() {
    let input = String::from("konstantinos.karachristos");

    let base64_representation: String = URL_SAFE.encode(input.as_bytes());
    println!("{} / {}", base64_representation, input);

    let color = get_color(input);
    println!("color: {:?}", color);
}

fn get_color(input: String) -> Color {
    let input_as_u8 = input.as_bytes();
    Color {
        red: input_as_u8[0],
        green: input_as_u8[1],
        blue: input_as_u8[2],
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
