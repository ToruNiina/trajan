use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

#[derive(Debug, Fail, PartialEq)]
pub enum ErrorKind {
    #[fail(display = "I/O Error")]
    Io,
    #[fail(display = "Invalid Integer Format")]
    ParseIntError,
    #[fail(display = "Invalid Float Format")]
    ParseFloatError,
    #[fail(display = "Invalid Format: {:?}", error)]
    Format{
        error: std::string::String
    },
}
impl std::cmp::Eq for ErrorKind {}

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

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
            inner: error.context(ErrorKind::ParseFloatError),
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Error {
        Error {
            inner: error.context(ErrorKind::ParseIntError),
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
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
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
        Error { inner }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compare_errorkind() {
        {
            let lhs = super::ErrorKind::Io;
            let rhs = super::ErrorKind::Io;
            assert_eq!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::ParseIntError;
            let rhs = super::ErrorKind::ParseIntError;
            assert_eq!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::ParseFloatError;
            let rhs = super::ErrorKind::ParseFloatError;
            assert_eq!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::Format{error: "test".to_string()};
            let rhs = super::ErrorKind::Format{error: "test".to_string()};
            assert_eq!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::Format{error: "test1".to_string()};
            let rhs = super::ErrorKind::Format{error: "test2".to_string()};
            assert_ne!(lhs, rhs);
        }

        {
            let lhs = super::ErrorKind::Io;
            let rhs = super::ErrorKind::ParseIntError;
            assert_ne!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::Io;
            let rhs = super::ErrorKind::ParseFloatError;
            assert_ne!(lhs, rhs);
        }
        {
            let lhs = super::ErrorKind::Io;
            let rhs = super::ErrorKind::Format{error: "test".to_string()};
            assert_ne!(lhs, rhs);
        }
    }

    #[test]
    fn from_std_io_error() {
        let e = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(err.kind(), &super::ErrorKind::Io);
    }

    #[test]
    fn from_std_num_parseinterror() {
        let e = "foo".parse::<i64>().unwrap_err();
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(err.kind(), &super::ErrorKind::ParseIntError);
    }

    #[test]
    fn from_std_num_parsefloaterror() {
        let e = "foo".parse::<f64>().unwrap_err();
        let err: super::Error = std::convert::From::from(e);
        assert_eq!(err.kind(), &super::ErrorKind::ParseFloatError);
    }
}
