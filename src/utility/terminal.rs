pub fn colour_text(text: &str, color: palette::Srgba) -> String {
    let rgba: [u8; 4] = palette::Srgba::into_format(color).into();
    format!(
        "\x1B[38;2;{};{};{}m{}\x1B[0m",
        rgba[0], rgba[1], rgba[2], text
    )
}
