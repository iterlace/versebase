// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
// https://learning-rust.github.io/docs/e7.custom_error_types.html


use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::fmt;


#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

pub enum ErrorKind {
    // 3rd-party errors wrappers
    Io,
    Parse,

    // 1-st party errors
    FilePointerCorrupt,
    AlreadyExists,
    NotFound,
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            Io => "io error",
            Parse => "parsing error",
            FilePointerCorrupt => "file pointer is corrupt",
            AlreadyExists => "already exists",
            NotFound => "record not found"
        }
    }
}


impl fmt::Debug for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}


impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_str())
    }
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            kind: ErrorKind::Io,
            message: e.to_string()
        }
    }
}


impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Error {
            kind: ErrorKind::Parse,
            message: error.to_string(),
        }
    }
}
