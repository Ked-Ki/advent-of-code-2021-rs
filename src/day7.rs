use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::cmp;

use crate::util::error::CatchAllError;

pub fn run(mut input_reader: BufReader<File>) {
    let mut input_str = String::new();
    input_reader.read_to_string(&mut input_str)
        .expect("could not read input");

	let (_, init_poss): (_, Vec<usize>) = parse::parse(&input_str)
        .expect("couldn't parse input");

    let result_a = day_7a(&init_poss).expect("error during part a");
    println!("Day 7; Part A: {}", result_a);

    let result_b = day_7b(&init_poss).expect("error during part b");
    println!("Day 7; Part B: {}", result_b);
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

fn distance(p1: usize, p2: usize) -> i64 {
    (p1 as i64 - p2 as i64).abs()
}

fn day_7a(init_poss: &[usize]) -> Result<i64,CatchAllError> {
    both_parts(init_poss, distance)
}


fn day_7b(init_poss: &[usize]) -> Result<i64,CatchAllError> {
    fn cost(p1: usize, p2: usize) -> i64 {
        let distance = distance(p1,p2);
        (distance * (distance + 1)) / 2
    }

    both_parts(init_poss, cost)
}

fn both_parts<F>(init_poss: &[usize], cost_calc: F) -> 
    Result<i64,CatchAllError> 
where
    F: Fn(usize, usize) -> i64
{
    let board_size = init_poss.iter().max()
        .ok_or(CatchAllError::new("no positions input".to_string()))?;
    let mut min_cost: i64 = i64::MAX;
    for end_pos in 0..=*board_size {
        let mut cur_cost = 0;
        for init_pos in init_poss {
            cur_cost += cost_calc(*init_pos,end_pos);
        }
        min_cost = cmp::min(min_cost, cur_cost);
    }

    Ok(min_cost)
}
