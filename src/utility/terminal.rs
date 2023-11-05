use palette::Srgba;

/// Print a title message to the terminal.
pub fn title(text: &str) -> String {
    format!("██{:█^40}██", format!(" {} ", text))
}

/// Print a heading message to the terminal.
pub fn heading(text: &str) -> String {
    format!("=={:=^40}==", format!(" {} ", text))
}

/// Print a sub-heading message to the terminal.
pub fn subheading(text: &str) -> String {
    format!("--{:-^40}--", format!(" {} ", text))
}

/// Print a coloured message to the terminal.
pub fn colour_text(text: &str, colour: Srgba) -> String {
    let rgba: [u8; 4] = Srgba::into_format(colour).into();
    format!(
        "\x1B[38;2;{};{};{}m{}\x1B[0m",
        rgba[0], rgba[1], rgba[2], text
    )
}
