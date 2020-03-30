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
    flag_regex: Option<String>,
}

struct Cli {
    args: Args,
}

impl Cli {
    fn process_stdin(&self) {
        let stdin = io::stdin();

        let maybe_regex_closure =
            self.args
                .flag_regex
                .clone()
                .and_then(|string_regex| {
                    Some(regex::Regex::new(&string_regex).unwrap_or_else(|e| {
                        panic!("The provided regex could not be parsed: {}", e)
                    }))
                })
                .and_then(|regex| Some(devicon_lookup::parsers::regex::parser_from_regex(regex)));

        for line_result in stdin.lock().lines() {
            let line: String = line_result.expect("Failed to read line from stdin");
            let line: Line = {
                let mut line_builder = LineBuilder::new(line);

                if self.args.flag_color {
                    line_builder.with_parser(&devicon_lookup::parsers::color::strip_color);
                }

                if let Some(regex_closure) = &maybe_regex_closure {
                    line_builder.with_parser(regex_closure);
                };

                line_builder.build()
            };

            match line.parse() {
                Ok(p) => p.print_with_symbol(),
                Err(e) => panic!("{}", e),
            };
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
