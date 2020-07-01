use nom::error::ParseError;
use std::error::Error as StdError;
use std::fmt::Display;
use std::io::Error as IoError;
use std::net::AddrParseError;

pub type Result<I> = std::result::Result<I, Error<I>>;
pub type IResult<I, O> = std::result::Result<(I, O), Error<I>>;

#[derive(Debug)]
pub struct Error<I> {
    kind: ErrorKind,
    error: nom::error::VerboseError<I>,
    incomplete: String,
}

impl<I> Error<I> {
    pub fn new(kind: ErrorKind) -> Error<I> {
        Error {
            kind,
            error: nom::error::VerboseError { errors: vec![] },
            incomplete: "".to_string(),
        }
    }

    pub fn new_nom_error(input: I, kind: nom::error::ErrorKind) -> Self {
        Self {
            kind: ErrorKind::NomParserError("Nom parser error".to_string()),
            error: nom::error::VerboseError::from_error_kind(input, kind),
            incomplete: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IoError(IoError),
    NomParserError(String),
    StringError(String),
    AddrParseError(AddrParseError),
    EmptyError,
}

impl<I: std::fmt::Debug> StdError for Error<I> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.kind {
            ErrorKind::IoError(ref e) => Some(e),
            ErrorKind::AddrParseError(ref e) => Some(e),
            ErrorKind::NomParserError(_) => None,
            ErrorKind::StringError(_) => None,
            ErrorKind::EmptyError => None,
        }
    }
}

impl<I> Display for Error<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::IoError(ref e) => e.fmt(f),
            ErrorKind::AddrParseError(ref e) => e.fmt(f),
            ErrorKind::NomParserError(ref e) => e.fmt(f),
            ErrorKind::StringError(ref e) => e.fmt(f),
            ErrorKind::EmptyError => write!(f, "Empty Error"),
        }
    }
}

impl<I> From<std::string::String> for Error<I> {
    fn from(s: std::string::String) -> Self {
        Error::new(ErrorKind::StringError(s))
    }
}

impl<I> From<IoError> for Error<I> {
    fn from(s: IoError) -> Self {
        Error::new(ErrorKind::IoError(s))
    }
}

impl<I> From<AddrParseError> for Error<I> {
    fn from(s: AddrParseError) -> Self {
        Error::new(ErrorKind::AddrParseError(s))
    }
}

impl<'a> From<Error<&'a str>> for Error<()> {
    fn from(s: Error<&'a str>) -> Self {
        Error::new(ErrorKind::StringError(format!(
            "kind:{:?},error:{:?}",
            s.kind, s.error
        )))
    }
}

impl<I: std::fmt::Debug> From<nom::Err<(I, nom::error::ErrorKind)>> for Error<I> {
    fn from(i: nom::Err<(I, nom::error::ErrorKind)>) -> Self {
        match i {
            nom::Err::Error(err) | nom::Err::Failure(err) => Error::new_nom_error(err.0, err.1),
            nom::Err::Incomplete(i) => Error::new(ErrorKind::StringError(format!(
                "Nom parser Incomplete error: {:?}",
                i
            ))),
        }
    }
}

impl<I> Into<nom::error::VerboseError<I>> for Error<I> {
    fn into(self) -> nom::error::VerboseError<I> {
        self.error
    }
}

impl<I: std::fmt::Debug> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Error::new_nom_error(input, kind)
    }

    fn append(input: I, kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other
            .error
            .errors
            .push((input, nom::error::VerboseErrorKind::Nom(kind)));
        other
    }

    fn from_char(input: I, c: char) -> Self {
        Self {
            kind: ErrorKind::EmptyError,
            error: nom::error::VerboseError::from_char(input, c),
            incomplete: "".to_string(),
        }
    }
}
