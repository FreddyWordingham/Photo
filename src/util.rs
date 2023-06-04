use colored::Colorize;
use std::{fmt::Debug, str::FromStr};
use termion::terminal_size;

/// Print a colourful title bar to the terminal.
#[inline]
pub fn title(title: &str) {
    if let Ok((width, _)) = terminal_size() {
        let term_width = width as usize;

        let title = title.to_uppercase();

        let (left_bar, right_bar) = if term_width < ((title.len() * 2) + 11) {
            (4, 4)
        } else {
            let left_bar = (term_width - (title.len() * 2) - 3) / 2;
            (left_bar, term_width - (title.len() * 2) - 3 - left_bar)
        };

        print!("{} ", "\u{2588}".repeat(left_bar));

        for (pos, ch) in title.chars().enumerate() {
            match pos % 6 {
                0 => print!(" {}", format!("{}", ch).bright_red().bold()),
                1 => print!(" {}", format!("{}", ch).bright_yellow().bold()),
                2 => print!(" {}", format!("{}", ch).bright_green().bold()),
                3 => print!(" {}", format!("{}", ch).bright_cyan().bold()),
                4 => print!(" {}", format!("{}", ch).bright_blue().bold()),
                5 => print!(" {}", format!("{}", ch).bright_magenta().bold()),
                _ => unreachable!(),
            }
        }

        println!("  {}", "\u{2588}".repeat(right_bar));
    } else {
        println!("{}", title.to_uppercase());
    }
}

/// Parse a string of the form "WxH" into a tuple of (width, height).
pub fn parse_resolution_string<T: FromStr>(resolution: &str) -> (T, T)
where
    <T as FromStr>::Err: Debug,
{
    let mut split = resolution.split('x');
    let width = split.next().unwrap().parse::<T>().unwrap();
    let height = split.next().unwrap().parse::<T>().unwrap();
    (width, height)
}
