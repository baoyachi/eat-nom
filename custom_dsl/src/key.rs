use nom::{InputTakeAtPosition, AsChar};
use nom::error::ErrorKind;

pub fn key(input: &str) -> nom::IResult<&str, &str> {
    input.split_at_position1_complete(
        |item|
            !(item.is_alphanum() || item.as_char() == '-' || item.as_char() == '_'),
        ErrorKind::Alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        let input = "hello_rust nom";
        let (input, out) = key(input).unwrap();
        assert_eq!(" nom", input);
        assert_eq!("hello_rust", out);

        let input = "hell123o-rust~nom";
        let (input, out) = key(input).unwrap();
        assert_eq!("~nom", input);
        assert_eq!("hell123o-rust", out);
    }
}