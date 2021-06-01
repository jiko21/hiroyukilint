#[macro_use]
mod hiroyuki;
extern crate clap;
extern crate exitcode;
extern crate counted_array;

use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file_path = matches.value_of("INPUT").unwrap();

    let f = File::open(file_path).expect("CANNOT OPEN FILE");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(line) => {

            },
            Err(_) => {
                eprintln!("Cannot read line @");
                std::process::exit(exitcode::DATAERR);
            }
        };
    }
}
