use std::io::{self, Write};
use ansi_term::Style;

pub use crate::file::File;
pub use crate::file_ext::FileExtensions;
pub use crate::devicon_lookup::icon::{FileIcon, Icons};
pub use crate::devicon_lookup::color::FileColours;
use crate::args::Args;

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
            file: File::new(&filename)
        })
    }
}


impl ParsedLine {
    fn symbol(&self) -> char {
        let file_icon = Box::new(FileExtensions);

        let icon = icon::find_exact_name(&self.file.name);

        if icon.is_none() && self.file.is_dir {
            return *icon::find_direcotry(&self.file.name)
                    .unwrap_or(&Icons::Dir.value());
        } if let Some(icon) = file_icon.custom_match(&self.file) { return icon; }
        else if icon.is_none() && self.file.ext.is_some() {
            return *icon::find_extension(&self.file.ext)
                    .unwrap_or(&Icons::File.value());
        }
        return *icon.unwrap_or(&Icons::File.value());
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
        return color::iconify_style(style.unwrap_or_default());
    }

    fn get_icon(&self, flag_iconcolor: bool) -> String {
        let mut icon = self.symbol().to_string();
        if flag_iconcolor {
            icon = self.color().paint(icon).to_string();
        }
        icon        
    }

    fn get_name(&self, flag_long: bool, flag_nameshort: bool) -> String {
        let name = 
            if flag_nameshort {
                File::short_path_part(&self.file.name.clone(), true)
            } else {
                self.file.name.clone()
            };
        if flag_long {
            color::main_color().paint(format!("{:<22}", name)).to_string()
        } else {
            name
        }
    }

    fn get_path(&self, flag_long: bool, flag_dirshort: bool, flag_dirshortreverse: bool) -> String {
        let path =
            if flag_dirshort || flag_dirshortreverse {
                self.file.short_path(flag_dirshortreverse)
            } else {
                self.file.path.clone()
            };

        if flag_long {
            color::detail_color().paint(path).to_string()
        } else {
            path
        }            
    }

    pub fn print_with_symbol(&self, args: &Args) {
        let icon = self.get_icon(args.flag_iconcolor);
        let name = self.get_name(args.flag_long, args.flag_nameshort);
        let path = self.get_path(args.flag_long, args.flag_dirshort, args.flag_dirshortreverse);

        let out = 
            if args.flag_long {
                format!("{} {}{}", icon, name, path )                
            } else {
                format!("{} {}{}", icon, path, name )
            };

        let mut s;
        if args.flag_fzf {
            s = format!("{}!{}", &self.file.full_path, out);
        } else {
            s = format!("{}", out);
        }

        if args.flag_substitute {
            s = format!("{}\n", self.original.replace(&self.file.full_path, &s));
        } else {
            s = s + "\n";
        }
        write_to_stdout(s.as_bytes())
    }
}

pub fn write_to_stdout(item: &[u8]) {
    let write_result = io::stdout().write_all(item);

    if write_result.is_err() {
        ::std::process::exit(0)
    }
}
