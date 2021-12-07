use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use log::debug;

use crate::util::error::CatchAllError;

pub fn run(mut input_reader: BufReader<File>) {
    let mut input_str = String::new();
    input_reader.read_to_string(&mut input_str)
        .expect("could not read input");

	let (_, parsed_lines) = parse::parse_lines(&input_str)
        .expect("couldn't parse input");

    debug!("\n{:?}", parsed_lines);

    let result_a = day_5a(&parsed_lines).expect("error during part a");
    println!("Day 5; Part A: {}", result_a);

    let result_b = day_5b(&parsed_lines).expect("error during part b");
    println!("Day 5; Part B: {}", result_b);
}

type Coord = (usize,usize);
type Line = (Coord, Coord);

mod parse {
    use super::Coord;
    use super::Line;

    use nom::{
        IResult,
        combinator::map,
        bytes::complete::tag,
        character::complete::{
            char,
            newline
        },
        error::VerboseError,
        multi::{
            many1
        },
        sequence::{
            terminated,
            separated_pair
        }
    };

    use crate::util::parse::{
        usize_parser
    };

    pub fn parse_lines(input: &str) -> IResult<&str, Vec<Line>, VerboseError<&str>> {
        fn parse_coord(input: &str) -> IResult<&str, Coord, VerboseError<&str>> {
            map(
                separated_pair(usize_parser, char(','), usize_parser),
                |(x,y)| (y,x) // input is x,y but grid is vec of rows so indexes are flipped
            )(input)
        }

        fn parse_line(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
            separated_pair(parse_coord, tag(" -> "), parse_coord)(input)
        } 

        many1(terminated(parse_line, newline))(input)
    }
}

fn covered_coords(line: Line) -> Vec<Coord> {
    fn range(start: usize, end: usize) -> Box<dyn Iterator<Item = usize>> {
        if start <= end {
            return Box::new((start..=end).into_iter());
        } else {
            return Box::new((end..=start).into_iter().rev());
        }
    }
    let mut out: Vec<Coord> = Vec::new();
    if is_diagonal(line) {
        for (i,j) in range(line.0.0,line.1.0).zip(range(line.0.1,line.1.1)) {
            out.push((i,j));
        }
    } else {
        for i in range(line.0.0,line.1.0) {
            for j in range(line.0.1,line.1.1) {
                out.push((i,j));
            }
        }
    }
    debug!("coords for line {:?}: {:?}", line, out);
    return out;
}

fn is_diagonal(line: Line) -> bool {
    line.0.0 != line.1.0 && line.0.1 != line.1.1
}


fn day_5a(lines: &[Line]) -> Result<usize,CatchAllError> {
    both_parts(lines, false)
}

fn day_5b(lines: &[Line]) -> Result<usize,CatchAllError> {
    both_parts(lines, true)
}

fn both_parts(lines: &[Line], include_diag: bool) -> Result<usize, CatchAllError> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let grid_size = 999; // input coords are <=3 digits
    //let grid_size = 10; // example coords are 1 digit
    for _ in 0..grid_size {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..grid_size {
            row.push(0);
        }
        grid.push(row);
    }

    for line in lines {
        if include_diag || !is_diagonal(*line) {
            for (i,j) in covered_coords(*line) {
                grid[i][j] = grid[i][j] + 1;
            }
            debug!("added line {:?}, grid is:\n{}", line, pretty_grid(&grid));
        }
    }

    debug!("grid after:\n{}", pretty_grid(&grid));

    return Ok(grid.iter()
            .flat_map(|r| r.iter())
            .filter(|&cell_count| cell_count > &1)
            .count());
}

fn pretty_grid(grid: &[Vec<i32>]) -> String {
    let mut out: String = String::from("");
    for row in grid {
        for cell in row {
            out.push_str(&cell.to_string());
        }
        out.push('\n')
    }

    return out;
}
