use std::error::Error;
use std::fmt;
use std::num;

#[derive(Debug, Eq, PartialEq)]
pub enum Expected {
    Base,
    Constant,
    Char(char),
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expected::Base => write!(f, "Base"),
            Expected::Constant => write!(f, "Constant"),
            Expected::Char(c) => write!(f, "character: {}", c),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ParseError {
    UnexpectedChar { pos: usize, c: char, exp: Expected },
    Float(String),
    UnexpectedEnd,
    UnconsumedInput(usize),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        "An error occured while parsing the expression"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseError::UnexpectedChar {
                ref pos,
                ref c,
                ref exp,
            } => write!(f, "Looking for {}, found {} at {}", exp, c, pos),
            &ParseError::Float(ref s) => f.write_fmt(format_args!("{}", s)),
            &ParseError::UnexpectedEnd => write!(f, "Unexpected end of input"),
            &ParseError::UnconsumedInput(p) => write!(f, "Unconsumed input starting at {}", p),
        }
    }
}

impl From<num::ParseFloatError> for ParseError {
    fn from(err: num::ParseFloatError) -> ParseError {
        ParseError::Float(format!("{}", err))
    }
}

pub type ParseResult<R> = Result<R, ParseError>;
