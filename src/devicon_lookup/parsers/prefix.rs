use crate::{devicon_lookup::ParserFn, ParserResult};

pub fn parser_from_prefix_delimiter(delimiter: String) -> impl ParserFn {
    move |input: String| -> ParserResult {
        match input.split(&delimiter).next() {
            Some(prefix) => Ok(prefix.to_string()),
            None => Err("Couldn't find the delimiter"),
        }
    }
}
