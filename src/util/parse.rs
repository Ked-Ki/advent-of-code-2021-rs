use nom::{
  IResult,
  combinator::map_res,
  error::{
      ParseError,
      VerboseError
  },
  sequence::{
      terminated,
      delimited,
//      preceded
  },
  character::complete::{
      multispace0,
      digit1,
      space0
  }
}; 

pub fn term_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> 
    impl FnMut(&'a str) -> IResult<&'a str, O, E> 
    where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    terminated(inner, multispace0)
}

/*
pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> 
    impl FnMut(&'a str) -> IResult<&'a str, O, E> 
    where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
*/

pub fn horz_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> 
    impl FnMut(&'a str) -> IResult<&'a str, O, E> 
    where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}

/*
pub fn prec_ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> 
    impl FnMut(&'a str) -> IResult<&'a str, O, E> 
    where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(multispace0, inner)
}
*/

pub fn int_parser<'a>() -> impl FnMut(&'a str) -> 
    IResult<&'a str, i32, VerboseError<&str>> 
{
    map_res(
        digit1,
        |out: &str| out.parse::<i32>()
    )
}

pub fn usize_parser<'a>(input: &'a str) -> IResult<&'a str, usize, VerboseError<&str>> 
{
    map_res(
        digit1,
        |out: &str| out.parse::<usize>()
    )(input)
}
