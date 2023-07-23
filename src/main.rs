#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::io::{self, BufRead};

mod devicon_lookup;
use devicon_lookup::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const USAGE: &str = include_str!("USAGE.txt");

mod lines;

#[derive(Debug, Deserialize)]
struct Args {
    flag_color: bool,
    flag_version: bool,
    flag_regex: Option<String>,
    flag_prefix: Option<String>,
}

struct Cli {
    args: Args,
}

impl Cli {
    fn process_stdin(&self) {
        let stdin = io::stdin();

        let maybe_regex_closure = self
            .args
            .flag_regex
            .clone()
            .map(|string_regex| {
                regex::Regex::new(&string_regex)
                    .unwrap_or_else(|e| panic!("The provided regex could not be parsed: {}", e))
            })
            .map(devicon_lookup::parsers::regex::parser_from_regex);

        let maybe_prefix_closure =
            self.args.flag_prefix.clone().map(|prefix| {
                devicon_lookup::parsers::prefix::parser_from_prefix_delimiter(prefix)
            });

        let lines_iter = lines::Lines::new(stdin.lock());

        for line_result in lines_iter {
            match line_result {
                Ok(line) => {
                    let line: Line = {
                        let mut line_builder = LineBuilder::new(line);

                        if self.args.flag_color {
                            line_builder.with_parser(&devicon_lookup::parsers::color::strip_color);
                        };

                        if let Some(prefix_closure) = &maybe_prefix_closure {
                            line_builder.with_parser(prefix_closure);
                        };

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
                Err(e) => {
                    // dbg!(e);
                }
            }
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
