use std::error::Error;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError")
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Clone)]
pub struct Reference {
    pub name: String,
    pub module: String,
    pub path: String,
}