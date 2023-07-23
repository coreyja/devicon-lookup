use crate::ParserResult;
use regex::Regex;

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

pub fn strip_color(input: String) -> ParserResult {
    Ok(ANSI_COLOR_REGEX.replace_all(&input, "").to_string())
}

// Make sure that `strip_color` implements `ParserFn`
#[cfg(test)]
const _: &dyn crate::devicon_lookup::ParserFn = &strip_color;
