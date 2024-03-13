#[derive(Debug, PartialEq)]
pub enum DateParseError {
    InvalidYear,
    InvalidMonth,
    InvalidDay,
    InvalidFormat,
}
