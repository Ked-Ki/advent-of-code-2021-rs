use std::fs::File;
use std::io::BufReader;

extern crate clap;
use clap::{Arg, App};

mod util;

use crate::util::error::CatchAllError;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use crate::day1::run as run_day_1;
use crate::day2::run as run_day_2;
use crate::day3::run as run_day_3;
use crate::day4::run as run_day_4;
use crate::day5::run as run_day_5;
use crate::day6::run as run_day_6;


fn main() -> Result<(), CatchAllError> {
    let matches = App::new("Advent of Code 2021")
        .arg(Arg::with_name("example")
             .short("e")
             .long("example")
             .help("Uses examples/day_<n>.txt instead of regular input"))
        .arg(Arg::with_name("day_number")
             .help("Sets which day to run"))
        .get_matches();

    let day_number: usize = matches.value_of("day_number")
        .ok_or(CatchAllError::new("expected day_number argument".to_string()))?
        .parse::<usize>()
        .map_err(|_| CatchAllError::new("day_number must be an int".to_string()))?;

    let use_example: bool = matches.is_present("example");

    let input_filename: &str = &format!("{}/day_{}.txt",
                                        if use_example { "examples" } else { "inputs"},
                                        day_number);
    let input_file = File::open(input_filename)
        .map_err(|_| CatchAllError::new(format!("input file {} not found", input_filename)))?;
    let input_reader = BufReader::new(input_file);

    env_logger::init();

    match day_number {
        1 => Ok(run_day_1(input_reader)),
        2 => Ok(run_day_2(input_reader)),
        3 => Ok(run_day_3(input_reader)),
        4 => Ok(run_day_4(input_reader)),
        5 => Ok(run_day_5(input_reader)),
        6 => Ok(run_day_6(input_reader)),
        _ => Err(CatchAllError::new("unknown day number".to_string()))
    }
}
