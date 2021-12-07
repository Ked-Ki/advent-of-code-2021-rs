use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::cmp::Ordering;

use crate::util::error::CatchAllError;

pub fn run(input_reader: BufReader<File>) {
    let input_iter = input_reader.lines();
    let input_vec_result: Result<Vec<Vec<Bit>>,_> = input_iter
        .map(parse_line)
        .collect();

    let input_vec = input_vec_result.expect("could not parse input");
    
    let result_a = day_3a(&input_vec);
    println!("Day 3; Part A: {}", result_a);

    let result_b = day_3b(&input_vec);
    println!("Day 3; Part B: {}", result_b);
}

#[derive(PartialEq,Eq,Copy,Clone)]
enum Bit {
    One,
    Zero,
}

impl Bit {
    fn negate(self) -> Bit {
        match self {
            Bit::One => Bit::Zero,
            Bit::Zero => Bit::One
        }
    }
}

fn parse_line(lr: Result<String,io::Error>) -> Result<Vec<Bit>,CatchAllError> {
    let l = lr
        .map_err(|err| CatchAllError::new(format!("io error reading input iter: {}", err.to_string())))?;

    let mut bit_vec: Vec<Bit> = Vec::new();

    for c in l.chars() {
        let r_bit: Result<Bit,CatchAllError> = match c {
            '1' => Ok(Bit::One),
            '0' => Ok(Bit::Zero),
             _  => Err(CatchAllError::new(
                     String::from("unexpected input (all chars should be 0 or 1)")))
        };

        bit_vec.push(r_bit?);
    }

    return Ok(bit_vec);
}

fn day_3a(input: &[Vec<Bit>]) -> i32 {
    let borrowed_vec: Vec<&Vec<Bit>> = input.iter().collect();

    let common_bits: Vec<Bit> = find_common_bits(&borrowed_vec[..]).iter()
        .map(|opt_b| *(opt_b.as_ref().unwrap_or(&Bit::One))) // ties are undefined behavior, just pick something
        .collect();

    let uncommon_bits: Vec<Bit> = common_bits.iter()
        .map(|b| b.negate())
        .collect();

    let gamma = bits_to_int(&common_bits);
    let epsilon = bits_to_int(&uncommon_bits);

    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);

    return gamma * epsilon;
}

// returns the most common bit for each position
fn find_common_bits(bit_vecs: &[&Vec<Bit>]) -> Vec<Option<Bit>> {
    let mut iter = bit_vecs.iter();

    let first = iter.next().expect("should have at least one reading");

    let mut accum: Vec<i32> = Vec::new();
    for bit in *first {
        match bit {
            Bit::One => accum.push(1),
            Bit::Zero => accum.push(-1)
        }
    }

    for vec in iter {
        for i in 0..vec.len() {
            match vec[i] {
                Bit::One => accum[i] = accum[i] + 1,
                Bit::Zero => accum[i] = accum[i] - 1
            }
        }
    }

    return accum.iter()
        .map(|i| match i.cmp(&0) {
            Ordering::Greater => Some(Bit::One),
            Ordering::Equal => None,
            Ordering::Less => Some(Bit::Zero)
        })
        .collect();
}

fn bits_to_int(bs: &[Bit]) -> i32 {
    let mut r = 0;
    for b in bs {
        r = r << 1;
        if let Bit::One = b {
            r = r + 1;
        }
    }
    return r;
}

fn day_3b(input: &[Vec<Bit>]) -> i32 {
   let oxy_rating = determine_rating(
       input,
       |b| match b {
           Some(b) => b,
           None => Bit::One
       });

   let co2_rating = determine_rating(
       input,
       |b| match b {
           Some(b) => b.negate(),
           None => Bit::Zero
       });

   println!("oxy_rating: {}", oxy_rating);
   println!("co2_rating: {}", co2_rating);

   return oxy_rating * co2_rating;
}

fn determine_rating(input: &[Vec<Bit>], criterion: fn(Option<Bit>) -> Bit) -> i32 {
   let mut candidates: Vec<&Vec<Bit>> = input.iter().collect();
   let mut index = 0;

   // undefined behavior for running out of numbers, so we skip bounds check for index
   while candidates.len() > 1 {
       let common_bits = find_common_bits(&candidates[..]);
       
       let keep_bit = criterion(common_bits[index]);

       candidates = candidates.iter()
           .map(|v| *v)
           .filter(|v| v[index] == keep_bit)
           .collect();

       index = index + 1;
   }

   let rating_bits = candidates[0];
   return bits_to_int(rating_bits);
}
