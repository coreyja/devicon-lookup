use crate::{devicon_lookup::ParserFn, ParserResult};
use regex::Regex;

pub fn parser_from_regex(regex: Regex) -> impl ParserFn {
    move |input: String| -> ParserResult {
        let captures = regex
            .captures(&input)
            .ok_or("Couldn't get captures from input")?;

        Ok(captures
            .get(1)
            .ok_or("The provided regex did not have a first capture group")?
            .as_str()
            .to_string())
    }
}
