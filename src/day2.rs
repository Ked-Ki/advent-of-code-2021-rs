use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fmt;

use crate::util::error::CatchAllError;

struct Command {
    direction: Direction,
    length: i32
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.direction, self.length)
    }
}

enum Direction {
    Forward,
    Down,
    Up
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f_str = match self {
            Direction::Forward => "forward",
            Direction::Down => "down",
            Direction::Up => "up"
        };
        write!(f, "{}", f_str)
    }
}

pub fn run(input_reader: BufReader<File>) {
    let input_iter = input_reader.lines();
    let input_vec_result: Result<Vec<Command>,_> = input_iter
        .map(parse_line)
        .collect();

    let input_vec = input_vec_result.expect("could not parse input");

    let result_a = day_2a(&input_vec);
    println!("Day 2; Part A: ({},{}) -> {}", result_a.0, result_a.1, result_a.0 * result_a.1);

    let result_b = day_2b(&input_vec);
    println!("Day 2; Part B: ({},{}) -> {}", result_b.0, result_b.1, result_b.0 * result_b.1);
}

fn parse_line(lr: Result<String,io::Error>) -> Result<Command,CatchAllError> {
    let l = lr
        .map_err(|err| CatchAllError::new(
                format!("io error reading input iter: {}", err.to_string())))?;
    let mut word_iter = l.split(char::is_whitespace);
    
    let mk_parse_err = || CatchAllError::new(
        "unexpected input. should be \"[forward|down|up] [int]\"".to_string());

    let dir_str = word_iter.next()
        .ok_or(mk_parse_err())?;
    let dir;

    if dir_str == "forward" {
        dir = Direction::Forward;
    } else if dir_str == "down" {
        dir = Direction::Down;
    } else if dir_str == "up" {
        dir = Direction::Up;
    } else {
        return Err(mk_parse_err());
    }

    let len_str = word_iter.next()
        .ok_or(mk_parse_err())?;
    let len = len_str.parse::<i32>()
        .map_err(|_| mk_parse_err())?;

    return Ok(Command {direction: dir, length: len});
}

fn day_2a(commands: &[Command]) -> (i32,i32) {
    let mut hor_pos = 0;
    let mut depth = 0;

    for command in commands.iter() {
        match command.direction {
            Direction::Forward => hor_pos = hor_pos + command.length,
            Direction::Down => depth = depth + command.length,
            Direction::Up => depth = depth - command.length,
        }
    }

    return (hor_pos, depth);
}

fn day_2b(commands: &[Command]) -> (i32,i32) {
    let mut hor_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands.iter() {
        match command.direction {
            Direction::Forward => {
                hor_pos = hor_pos + command.length;
                depth = depth + aim * command.length;
            },
            Direction::Down => aim = aim + command.length,
            Direction::Up => aim = aim - command.length,
        }
    }

    return (hor_pos, depth);
}
