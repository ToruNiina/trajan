//! error handling.
use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

/// An enum to represent an error occured in the library.
#[derive(Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "I/O Error")]
    Io,
    #[fail(display = "Parse Value Error")]
    ParseError,
    #[fail(display = "Invalid Format: {:?}", error)]
    InvalidFormat{
        error: std::string::String
    },
}
impl std::cmp::Eq for ErrorKind {}

/// An error type besed on Faliure library.
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

/// A type alias to handle an error occured in the library.
pub type Result<T> = std::result::Result<T, Error>;

/* ----------- conversion between errors ----------- */

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl std::convert::From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Error {
        Error {
            inner: error.context(ErrorKind::ParseError),
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Error {
        Error {
            inner: error.context(ErrorKind::ParseError),
        }
    }
}

impl std::convert::From<std::string::ParseError> for Error {
    fn from(error: std::string::ParseError) -> Error {
        Error {
            inner: error.context(ErrorKind::ParseError),
        }
    }
}

/* ----------- failure boilerplate ----------- */


impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    /// Constructs `Error`.
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error{inner}
    }
    /// get ErrorKind of the contained error type.
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
    /// Constructs `trajan::error::Error` from `std::string::String` that
    /// represents a portion of an input that is formatted in the invalid way.
    pub fn invalid_format(s: std::string::String) -> Error {
        Error{inner: failure::Context::new(ErrorKind::InvalidFormat{error: s})}
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error {inner}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_std_io_error() {
        let e = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(*err.kind(), super::ErrorKind::Io);
    }

    #[test]
    fn from_std_num_parseinterror() {
        let e = "foo".parse::<i64>().unwrap_err();
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(*err.kind(), super::ErrorKind::ParseError);
    }

    #[test]
    fn from_std_num_parsefloaterror() {
        let e = "foo".parse::<f64>().unwrap_err();
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(*err.kind(), super::ErrorKind::ParseError);
    }

    #[test]
    fn from_invalid_format() {
        let err = super::Error::invalid_format("test".to_string());
        assert_eq!(*err.kind(), super::ErrorKind::InvalidFormat{error: "test".to_string()});
    }
}
