#[macro_use]
mod hiroyuki;
mod util;
extern crate clap;
extern crate counted_array;
extern crate exitcode;

use crate::hiroyuki::checker::Checker;
use crate::util::util::read_file_to_array;
use clap::{load_yaml, App};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let yaml = load_yaml!("./cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let file_path = matches.value_of("INPUT").unwrap();
    let blacklist_path = matches.value_of("BLACKLIST");

    let f = File::open(file_path).expect("CANNOT OPEN FILE");
    let reader = BufReader::new(f);
    let mut blacklists = match blacklist_path {
        Some(blacklist) => match File::open(blacklist) {
            Ok(file) => {
                let buf = BufReader::new(file);
                read_file_to_array(buf)
            }
            Err(_) => {
                vec![]
            }
        },
        _ => {
            vec![]
        }
    };

    println!("{:?}", blacklists);

    let mut hiroyuki_checker = Checker::new(reader, &mut blacklists);
    hiroyuki_checker.print();
}
