use palette::Srgba;

pub fn colour_text(text: &str, colour: Srgba) -> String {
    let rgba: [u8; 4] = Srgba::into_format(colour).into();
    format!(
        "\x1B[38;2;{};{};{}m{}\x1B[0m",
        rgba[0], rgba[1], rgba[2], text
    )
}
