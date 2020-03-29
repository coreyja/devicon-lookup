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

        let maybe_regex = self
            .args
            .flag_regex
            .clone()
            .and_then(|string_regex| regex::Regex::new(&string_regex).ok());

        for line_result in stdin.lock().lines() {
            let line: String = line_result.expect("Failed to read line from stdin");
            let line: Line = {
                let mut line_builder = LineBuilder::new(line);

                if self.args.flag_color {
                    line_builder.with_parser(&strip_color);
                }

                // if let Some(regex) = maybe_regex {
                //     line_builder.with_parser(parser_for_regex(&regex))
                // }

                line_builder.build()
            };

            let parsed = line.parse();
            parsed.unwrap().print_with_symbol();
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
