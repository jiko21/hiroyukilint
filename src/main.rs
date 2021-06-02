#[macro_use]
mod hiroyuki;
extern crate clap;
extern crate exitcode;
extern crate counted_array;

use crate::hiroyuki::check;
use crate::hiroyuki::print;
use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file_path = matches.value_of("INPUT").unwrap();

    let f = File::open(file_path).expect("CANNOT OPEN FILE");
    let reader = BufReader::new(f);

    let mut code_line = 1;
    let mut error_count = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let points = check(code_line, &line);
                error_count += points.len();
                for point in points {
                    print(code_line, &line, point);
                }
                code_line += 1;
            },
            Err(_) => {
                eprintln!("Cannot read line @ {}", code_line);
                std::process::exit(exitcode::DATAERR);
            }
        };
    }
}
