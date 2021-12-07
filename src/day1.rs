use std::io;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run(input_reader: BufReader<File>) {
    let input_iter = input_reader.lines();
    let input_vec_result: Result<Vec<i32>,_> = input_iter
        .map(parse_line)
        .collect();

    let input_vec = input_vec_result.expect("could not parse input");
    
    let result_a = day_1a(&input_vec);
    println!("Day 1; Part A: {}", result_a);

    let result_b = day_1b(&input_vec, input_vec.len());
    println!("Day 1; Part B: {}", result_b);
}

fn parse_line(lr: Result<String,io::Error>) -> Result<i32,Box<dyn Error>> {
    return Ok(lr?.parse::<i32>()?);
}

fn day_1a(depths: &[i32]) -> i32 {
    let mut depth_iter = depths.iter();
    let mut prev_depth = depth_iter.next()
        .expect("depth array should have at least one element");
    let mut incr_count = 0;
    for cur_depth in depth_iter {
        if cur_depth > prev_depth {
            incr_count = incr_count + 1;
        }
        prev_depth = cur_depth;
    }
    return incr_count;
}

fn day_1b(depths: &[i32], len: usize) -> i32 {
    let mut cur_window = 0;
    let mut incr_count = 0;
    let depth_slice = &depths;
    for n in 0..3 {
        cur_window = cur_window + depth_slice[n];
    }
    for n in 3..len {
        let new_window = cur_window + depth_slice[n] - depth_slice[n-3];
        if new_window > cur_window {
            incr_count = incr_count + 1;
        }
        cur_window = new_window;
    }
    return incr_count;
}
