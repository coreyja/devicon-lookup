use std::io::{self, Write};
use ansi_term::Style;

pub use crate::file::File;
pub use crate::file_ext::FileExtensions;
pub use crate::devicon_lookup::icon::FileIcon;
pub use crate::devicon_lookup::color::FileColours;

pub mod icon;
pub mod color;
pub mod parsers;

pub trait ParserFn: Fn(String) -> ParserResult {}

impl<T: Fn(String) -> ParserResult> ParserFn for T {}

pub type ParserResult = Result<String, &'static str>;
pub type Parser = dyn ParserFn;

pub struct ParsedLine {
    original: String,
    file: File,
}

pub struct Line<'a> {
    pub original: String,
    pub filename_parsers: Vec<&'a Parser>,
}

pub struct LineBuilder<'a> {
    original: String,
    filename_parsers: Vec<&'a Parser>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(original: String) -> Self {
        Self {
            original,
            filename_parsers: vec![],
        }
    }

    pub fn with_parser(&mut self, parser: &'a Parser) {
        self.filename_parsers.push(parser);
    }

    pub fn build(self) -> Line<'a> {
        Line {
            original: self.original,
            filename_parsers: self.filename_parsers,
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(self) -> Result<ParsedLine, &'static str> {
        let mut curr: ParserResult = Ok(self.original.clone());

        for parser in self.filename_parsers.iter() {
            curr = parser(curr?)
        }

        let filename = curr?;
        Ok(ParsedLine {
            original: self.original,
            file: File::from_filename(filename)
        })
    }
}

pub const DEFAULT_FILE_SYMBOL: char = '\u{f15b}';   // 
pub const DEFAULT_DIR_SYMBOL: char = '\u{f07c}';   // 

impl ParsedLine {
    fn symbol(&self) -> char {
        let file_icon = Box::new(FileExtensions);

        let icon = icon::find_exact_name(&self.file.name);

        if icon.is_none() && self.file.is_dir {
            return *icon::find_direcotry(&self.file.name)
                    .unwrap_or(&DEFAULT_DIR_SYMBOL);
        } if let Some(icon) = file_icon.custom_match(&self.file) { return icon; }
        else if icon.is_none() && self.file.ext.is_some() {
            return *icon::find_extension(&self.file.ext)
                    .unwrap_or(&DEFAULT_FILE_SYMBOL);
        }
        return *icon.unwrap_or(&DEFAULT_FILE_SYMBOL);
    }

    fn color(&self) -> Style {
        let file_color = Box::new(FileExtensions);
        let style: Option<Style>;
        if self.file.is_dir {
            style = file_color.color_dir(&self.file);
        }
        else {
            style = file_color.color_file(&self.file);
        }
        return file_color.iconify_style(style.unwrap_or_default());
    }

    pub fn print_with_symbol(&self) {
        let icon = self.symbol().to_string();
        let colored_icon = self.color().paint(icon);
        let s = format!("{} {}\n", colored_icon, self.original);
        write_to_stdout(s.as_bytes())
    }
}

pub fn write_to_stdout(item: &[u8]) {
    let write_result = io::stdout().write_all(item);

    if write_result.is_err() {
        ::std::process::exit(0)
    }
}
