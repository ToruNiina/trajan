#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Invalid Format")]
    Format,
}

/* ----------- failure boilerplate ----------- */

use std::fmt;
use std::fmt::Display;
use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

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

/* ----------- conversion between errors ----------- */

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error {
            inner: error.context(ErrorKind::Io),
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Error {
        Error {
            inner: error.context(ErrorKind::Format),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Error {
        Error {
            inner: error.context(ErrorKind::Format),
        }
    }
}
