use std::io::BufReader;
use std::io::Read;
use std::fs::File;

use log::debug;

use nom::{
    Finish,
    IResult,
    character::complete::{
        char,
        newline
    },
    error::{
        context,
        VerboseError,
        convert_error
    },
    multi::{
        separated_list0,
        many1
    },
    sequence::{
        terminated,
    }
};

use crate::util::parse::{
    term_ws,
    horz_ws,
    int_parser
};

use crate::util::error::CatchAllError;

pub fn run(mut input_reader: BufReader<File>) {
    let mut input_str = String::new();
    input_reader.read_to_string(&mut input_str)
        .expect("could not read input");
    
    let (unparsed, draws) = parse_draws(&input_str)
        .expect("could not parse draws");

    debug!("draws: {:?}", draws);
    debug!("unparsed: {}", unparsed);

    let board_parse_result = separated_list0(newline,parse_board)(&unparsed).finish();
    if board_parse_result.is_err() {
        println!("{}", convert_error(unparsed, board_parse_result.err().unwrap()));
        return;
    }
    let (unparsed2, boards) = board_parse_result.ok().unwrap();

    debug!("boards: {:?}", boards);
    debug!("unparsed2: {}", unparsed2);
    
    let result_a = day_4a(&draws, &boards).expect("error during part a");
    println!("Day 4; Part A: {}", result_a);

    let result_b = day_4b(&draws, &boards).expect("error during part b");
    println!("Day 4; Part B: {}", result_b);
}

fn parse_draws(input: &str) -> IResult<&str, Vec<i32>, VerboseError<&str>> {
    term_ws(
        separated_list0(
            char(','), 
            int_parser()
        )
    )(input)
}

type Board = Vec<Vec<i32>>;

fn parse_board(input: &str) -> IResult<&str, Board, VerboseError<&str>> {
    fn parse_row(input: &str) -> IResult<&str, Vec<i32>, VerboseError<&str>> {
        context(
            "parse_row",
            many1(horz_ws(int_parser()))
        )(input)
    }
    terminated(separated_list0(newline,parse_row), newline)(input)
}

fn day_4a(draws: &[i32], boards: &[Board]) -> Result<i32,CatchAllError> {
    let mut board_states: Vec<(&Board, BoardCounts)> = boards.iter()
        .map(|b| (b,BoardCounts::new()))
        .collect();

    let mut winning_board_idx = 0;
    let mut winning_draw = 0;

    'outer: for draw in draws {
        for i in 0..board_states.len() {
            let board = board_states[i].0;
            let state = &mut (board_states[i].1);
            
            let opt_idx = draw_idx(*draw, board, (*board).len());
            match opt_idx {
                Some(idx) => {
                    state.track_draw(idx);
                    if state.is_win() {
                        winning_board_idx = i;
                        winning_draw = *draw;
                        break 'outer;
                    }
                },
                None => ()
            }
        }
    };

    let winning_board_state = &board_states[winning_board_idx];

    debug!("winning board: {:?}", winning_board_state.0);
    debug!("state: {:?}", winning_board_state.1);
    debug!("pretty:\n{}", winning_board_state.1.pretty(winning_board_state.0));


    let winning_score = winning_board_state.1.compute_score(winning_board_state.0);

    return Ok(winning_score * winning_draw);
}

fn draw_idx(draw: i32, board: &[Vec<i32>], board_len: usize) ->
    Option<(usize,usize)>
{
    for i in 0..board_len {
        for j in 0..board[i].len() {
            if board[i][j] == draw {
                return Some((i,j));
            }
        }
    }
    return None;
}

#[derive(Debug)]
struct BoardCounts {
    rows: Vec<i32>,
    cols: Vec<i32>,
    diags: Vec<i32>,
    all_draws: Vec<(usize,usize)>
}
    
impl BoardCounts {
    fn new() -> BoardCounts {
        BoardCounts{ 
            rows: vec![0,0,0,0,0], 
            cols: vec![0,0,0,0,0], 
            diags: vec![0,0],
            all_draws: Vec::new()
        }
    }

    fn track_draw(&mut self, draw: (usize,usize)) {
        self.rows[draw.0] = self.rows[draw.0] + 1;
        self.cols[draw.1] = self.cols[draw.1] + 1;
        /*
        if draw.0 == draw.1 {
            self.diags[0] = self.diags[0] + 1;
        }
        if draw.0 + draw.1 == 5 {
            self.diags[1] = self.diags[1] + 1;
        }
        */
        self.all_draws.push(draw);
    }

    fn is_win(&self) -> bool {
        fn check_vec(v: &[i32]) -> bool {
            v.iter().any(|&c| c >= 5)
        }
        check_vec(&self.rows) || check_vec(&self.cols) // || check_vec(&self.diags)
    }

    fn compute_score(&self, board: &Board) -> i32 {
        let mut board_total: i32 = board.iter()
            .flat_map(|row| row.iter())
            .sum();

        for (i,j) in &self.all_draws {
            board_total = board_total - board[*i][*j]
        }

        return board_total;
    }

    fn pretty(&self, board: &Board) -> String {
        let mut out: String = String::from("");
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if self.all_draws.contains(&(i,j)) {
                    out.push_str(&format!("({:^2})", board[i][j]));
                } else {
                    out.push_str(&format!("{:^4}", board[i][j]));
                }
            }
            out.push('\n');
        }
        return out;
    }
}
    
fn day_4b(draws: &[i32], boards: &[Board]) -> Result<i32,CatchAllError> {
    let mut board_states: Vec<(&Board, BoardCounts)> = boards.iter()
        .map(|b| (b,BoardCounts::new()))
        .collect();

    let num_boards = board_states.len();

    let mut losing_board_idx = 0;
    let mut final_draw = 0;

    let mut num_won = 0;
    let mut win_idxs = Vec::with_capacity(num_boards);
    for _ in 0..num_boards {
        win_idxs.push(false);
    }

    'outer: for draw in draws {
        for i in 0..board_states.len() {
            let board = board_states[i].0;
            let state = &mut (board_states[i].1);
            
            let opt_idx = draw_idx(*draw, board, (*board).len());
            match opt_idx {
                Some(idx) => {
                    state.track_draw(idx);
                    if !win_idxs[i] && state.is_win() {
                        win_idxs[i] = true;
                        num_won = num_won + 1;
                        if num_won == num_boards {
                            losing_board_idx = i;
                            final_draw = *draw;
                            break 'outer
                        }
                    }
                },
                None => ()
            }
        }
    };


    let losing_board_state = &board_states[losing_board_idx];
    let losing_score = losing_board_state.1.compute_score(losing_board_state.0);

    debug!("pretty loser:\n{}", losing_board_state.1.pretty(losing_board_state.0));

    return Ok(losing_score * final_draw);
}
