pub type EResult<I> = std::result::Result<I, ErrorKind>;

#[derive(Debug)]
pub enum ErrorKind {
    StringError(String),
    NomError(String),
}

impl std::fmt::Display for ErrorKind {
    #[cfg_attr(tarpaulin, skip)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ErrorKind::StringError(ref e) => e.fmt(f),
            ErrorKind::NomError(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ErrorKind {
    #[cfg_attr(tarpaulin, skip)]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            ErrorKind::StringError(ref _e) => None,
            ErrorKind::NomError(ref _e) => None,
        }
    }
}

impl From<std::net::AddrParseError> for ErrorKind {
    #[cfg_attr(tarpaulin, skip)]
    fn from(s: std::net::AddrParseError) -> Self {
        ErrorKind::StringError(s.to_string())
    }
}


impl nom::error::ParseError<&str> for ErrorKind {
    #[cfg_attr(tarpaulin, skip)]
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        ErrorKind::NomError(format!("input:[{}],kind:[{:?}]", input, kind))
    }

    #[cfg_attr(tarpaulin, skip)]
    fn append(_input: &str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl From<nom::Err<(&str, nom::error::ErrorKind)>> for ErrorKind {
    #[cfg_attr(tarpaulin, skip)]
    fn from(i: nom::Err<(&str, nom::error::ErrorKind)>) -> Self {
        match i {
            nom::Err::Error(err) | nom::Err::Failure(err) => {
                ErrorKind::NomError(format!("input:[{}],kind:[{:?}]", err.0, err.1))
            }
            nom::Err::Incomplete(i) => {
                ErrorKind::StringError(format!("nom parser Incomplete error: {:?}", i))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::take_until;

    #[test]
    fn test_until_eof_ok() -> EResult<()> {
        fn until_eof(s: &str) -> nom::IResult<&str, &str> {
            take_until("eof")(s)
        }

        let (x, y) = until_eof("hello, worldeof")?;
        assert_eq!(x, "eof");
        assert_eq!(y, "hello, world");
        Ok(())
    }

    #[test]
    fn test_until_eof_error() -> EResult<()> {
        fn until_eof(s: &str) -> nom::IResult<&str, &str> {
            take_until("e1of")(s)
        }

        match until_eof("hello, worldeof") {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
        Ok(())
    }
}
