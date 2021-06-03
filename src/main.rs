#[macro_use]
mod hiroyuki;
extern crate clap;
extern crate counted_array;
extern crate exitcode;

use crate::hiroyuki::checker::Checker;
use clap::{load_yaml, App};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let yaml = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file_path = matches.value_of("INPUT").unwrap();

    let f = File::open(file_path).expect("CANNOT OPEN FILE");
    let reader = BufReader::new(f);

    let mut hiroyuki_checker = Checker::new(reader);
    hiroyuki_checker.print();
}
