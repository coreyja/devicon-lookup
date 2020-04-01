use crate::ParserResult;
use regex::Regex;

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

pub fn strip_color(input: ParserResult) -> ParserResult {
    Ok(ANSI_COLOR_REGEX.replace_all(&input?, "").to_string())
}
