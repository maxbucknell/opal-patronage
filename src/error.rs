use std::fmt;
use std::error;

#[derive(Debug)]
pub enum ErrorKind {
    CouldNotParseDate(String),
    CouldNotParseNumericComponent(usize, String),
    InvalidMonth(u16),
    InvalidDateForMonth(u16, u16),
    NoSingleDate(u16, u16, u16)
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind
        }
    }

    pub fn exit_code(&self) -> i32 {
        match &self.kind {
            ErrorKind::CouldNotParseDate(_) => exitcode::USAGE,
            ErrorKind::CouldNotParseNumericComponent(_, _) => exitcode::USAGE,
            ErrorKind::InvalidMonth(_) => exitcode::USAGE,
            ErrorKind::InvalidDateForMonth(_, _) => exitcode::USAGE,
            ErrorKind::NoSingleDate(_, _, _) => exitcode::USAGE,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::CouldNotParseDate(d) => write!(f, "Could not extract components from date string '{d}'."),
            ErrorKind::CouldNotParseNumericComponent(i, c) => write!(f, "Could not parse number from component '{c}' at index {i}."),
            ErrorKind::InvalidMonth(m) => write!(f, "Month {m} is not a valid month."),
            ErrorKind::InvalidDateForMonth(m, d) => write!(f, "Month {m} does not have {d} days in it"),
            ErrorKind::NoSingleDate(y, m, d) => write!(f, "{y}-{m}-{d} could not be converted into a single date.")
        }
    }
}

impl error::Error for Error {}
