use crate::ParserResult;

pub fn parser_from_prefix_delimiter(delimiter: String) -> impl Fn(ParserResult) -> ParserResult {
    move |input: ParserResult| -> ParserResult {
        let input = input?;

        match input.split(&delimiter).next() {
            Some(prefix) => Ok(prefix.to_string()),
            None => Err("Couldn't find the delimiter"),
        }
    }
}
