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

impl Cli {
    fn line_to_symbolable(&self, line: String) -> Box<Symbolable> {
        if self.args.flag_color {
            ColoredLine::boxed(line)
        } else {
            Line::boxed(line)
        }
    }

    fn process_stdin(&self) {
        let stdin = io::stdin();
        for line_result in stdin.lock().lines() {
            let line = line_result.expect("Failed to read line from stdin");
            let filename = self.line_to_symbolable(line);
            filename.print_with_symbol();
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