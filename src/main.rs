#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::io::{self, BufRead};

mod devicon_lookup;

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
    fn process_line(&self, line: &str) {
        let filename = String::from(line);
        let file_name_to_parse = if self.args.flag_color {
            devicon_lookup::strip_ansi_codes(&filename)
        } else {
            filename.clone()
        };
        let extension = devicon_lookup::get_extension_from_filename(&file_name_to_parse);
        let symbol = match extension {
            Some(extension) => devicon_lookup::get_symbol_from_extension(extension),
            None => devicon_lookup::get_default_symbol(),
        };
        println!("{} {}", symbol, filename);
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
