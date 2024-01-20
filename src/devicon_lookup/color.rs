pub use crate::file::File;
pub use crate::file_ext::FileExtensions;
use ansi_term::Colour::*;
use ansi_term::Style;

pub fn iconify_style(style: Style) -> Style {
    style
        .background
        .or(style.foreground)
        .map(Style::from)
        .unwrap_or_default()
}

pub fn detail_color() -> Style {
    iconify_style(Fixed(244).normal())
}

pub fn main_color() -> Style {
    iconify_style(White.normal())
}
