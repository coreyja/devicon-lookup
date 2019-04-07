#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::io::{self, BufRead};

mod devicon_lookup;
use devicon_lookup::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = include_str!("USAGE.txt");

#[derive(Debug, Deserialize)]
struct Args {
    flag_color: bool,
    flag_version: bool,
}

struct Cli {
    args: Args,
}

trait Symbolable {
    fn to_parse(&self) -> &str;
    fn to_print(&self) -> &str;

    fn extension(&self) -> Option<&str> {
        get_extension_from_filename(self.to_parse())
    }

    fn symbol(&self) -> &str {
        match self.extension() {
            Some(extension) => get_symbol_from_extension(extension),
            None => get_default_symbol(),
        }
    }

    fn print_with_symbol(&self) {
        println!("{} {}", self.symbol(), self.to_print());
    }
}

struct Line {
    line: String,
}

impl Symbolable for Line {
    fn to_parse(&self) -> &str {
        &self.line
    }
    fn to_print(&self) -> &str {
        &self.line
    }
}

struct ColoredLine {
    line: String,
    stripped_line: String,
}

impl ColoredLine {
    fn new(line: &str) -> ColoredLine {
        let stripped_line = strip_ansi_codes(&line);
        ColoredLine {
            line: String::from(line),
            stripped_line: stripped_line,
        }
    }
}

impl Symbolable for ColoredLine {
    fn to_parse(&self) -> &str {
        &self.stripped_line
    }
    fn to_print(&self) -> &str {
        &self.line
    }
}

impl Cli {
    fn process_line(&self, line: &str) {
        let filename: Box<Symbolable> = if self.args.flag_color {
            Box::new(ColoredLine::new(line))
        } else {
            Box::new(Line {
                line: String::from(line),
            })
        };
        filename.print_with_symbol();
    }

    fn process_stdin(&self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.process_line(&line.unwrap());
        }
    }

    fn run(&self) {
        if self.args.flag_version {
            println!("Dev Icon Lookup v{}", VERSION);
        } else {
            self.process_stdin();
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let cli: Cli = Cli { args };
    cli.run();
}
