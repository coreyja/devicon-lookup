use crate::ParserResult;
use regex::Regex;

pub fn parser_from_regex(regex: Regex) -> impl Fn(ParserResult) -> ParserResult {
    move |input: ParserResult| {
        Ok(regex
            .captures(&input?)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string())
    }
}
