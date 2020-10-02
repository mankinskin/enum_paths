extern crate derive_as_path;
pub use derive_as_path::*;
use std::str::FromStr;

pub trait AsPath {
    fn as_path(self) -> String;
}
impl<T: ToString> AsPath for T {
    fn as_path(self) -> String {
        format!("/{}", self.to_string())
    }
}

pub trait ParsePath: AsPath + Sized {
    fn parse_path(route: &str) -> Result<Self, ParseError>;
}
#[derive(Debug)]
pub enum ParseError {
    FromStr,
    NoMatch,
    By(String, Box<ParseError>),
    RemainingSegments,
}
impl<T: FromStr + ToString + AsPath> ParsePath for T {
    fn parse_path(path: &str) -> Result<Self, ParseError> {
        path.trim_start_matches("/")
            .parse::<T>()
            .map_err(|_| ParseError::FromStr)
    }
}

pub trait Named {
    fn get_name(&self) -> String;
}
impl<T: ToString> Named for T {
    fn get_name(&self) -> String {
        self.to_string()
    }
}
