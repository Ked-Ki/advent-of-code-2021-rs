use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::collections::VecDeque;

use log::debug;

use crate::util::error::CatchAllError;

pub fn run(mut input_reader: BufReader<File>) {
    let mut input_str = String::new();
    input_reader.read_to_string(&mut input_str)
        .expect("could not read input");

	let (_, init_fish) = parse::parse(&input_str)
        .expect("couldn't parse input");

    debug!("init_fish: {:?}", init_fish);

    let result_a = day_6a(&init_fish).expect("error during part a");
    println!("Day 6; Part A: {}", result_a);

    let result_b = day_6b(&init_fish).expect("error during part b");
    println!("Day 6; Part B: {}", result_b);
}

mod parse {
    use nom::{
        IResult,
        character::complete::char,
        error::VerboseError,
        multi::separated_list0
    };

    use crate::util::parse::usize_parser;

    pub fn parse(input: &str) -> IResult<&str, Vec<usize>,VerboseError<&str>> {
        separated_list0(char(','), usize_parser)(input)
    }
}

#[derive(Debug)]
struct Population {
    counters: VecDeque<i64>,
    new_idx: usize,
    reset_idx: usize
}

impl Population {
    fn new(init_fish: &[usize], cycle_len: usize, new_cycle_len: usize) -> Population {
        let mut counters: VecDeque<i64> = VecDeque::with_capacity(new_cycle_len);
        (0..new_cycle_len).for_each(|_| counters.push_back(0));

        for &fish in init_fish {
            counters[fish] = counters[fish] + 1;
        }
        Population { counters, new_idx: new_cycle_len - 1, reset_idx: cycle_len - 1 }
    }

    fn sim_step(&mut self) {
        let num_breeding = self.counters.pop_front().expect("counters should never be empty");
        self.counters.push_back(num_breeding);
        self.counters[self.reset_idx] += num_breeding;
    }

    fn simulate_for(&mut self, days: i32) {
        for _ in 0..days {
            self.sim_step()
        }
    }

    fn count_fish(&self) -> i64 {
        self.counters.iter().sum()
    }
}

fn day_6a(init_fish: &[usize]) -> Result<i64,CatchAllError> {
    let mut population = Population::new(init_fish, 7, 9);
    population.simulate_for(80);
    Ok(population.count_fish())
}

fn day_6b(init_fish: &[usize]) -> Result<i64,CatchAllError> {
    let mut population = Population::new(init_fish, 7, 9);
    population.simulate_for(256);
    Ok(population.count_fish())
}
