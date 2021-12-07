use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CatchAllError {
    reason: String
}

impl CatchAllError {
    pub fn new(reason: String) -> CatchAllError {
        CatchAllError{reason: reason}
    }
}

impl fmt::Display for CatchAllError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CatchAllError")
    }
}

impl <'a> Error for CatchAllError {}

