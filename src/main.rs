use base64::{engine::general_purpose::URL_SAFE, Engine as _};

fn main() {
    let input = String::from("konstantinos.karachristos");

    let base64_representation: String = URL_SAFE.encode(input.as_bytes());
    println!("{} / {}", base64_representation, input);
}
