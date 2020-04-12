use nom::error::ErrorKind;
use nom::InputTakeAtPosition;
use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::branch::alt;

pub fn ip(input: &str) -> nom::IResult<&str, &str> {
    input.split_at_position1_complete(
        |item| {
            match item as u8 {
                b'0'..=b'9' => false,
                b'A'..=b'F' | b'a'..=b'f' => false,
                b':' | b'.' => false,
                _ => true,
            }
        },
        ErrorKind::Char,
    )
}

pub fn mask(input: &str) -> nom::IResult<&str, &str> {
    ip(input)
}

pub fn ip_mask(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let split = alt((tag("/"), tag("-"), tag(" "),tag("~")));
    let (input, (ip, _, mask)) = tuple((ip, split, mask))(input)?;
    Ok((input, (ip, mask)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::*;

    #[test]
    fn test_ip_mask() -> Result<()> {
        let (input, (ip, mask)) = ip_mask("127.0.0.1/255.255.0.0").unwrap();
        assert_eq!(input, "");
        assert_eq!(ip, "127.0.0.1");
        assert_eq!(mask, "255.255.0.0");

        let (input, (ip, mask)) = ip_mask("127.0.0.1-255.255.0.0").unwrap();
        assert_eq!(input, "");
        assert_eq!(ip, "127.0.0.1");
        assert_eq!(mask, "255.255.0.0");

        let (input, (ip, mask)) = ip_mask("127.0.0.1 255.255.0.0").unwrap();
        assert_eq!(input, "");
        assert_eq!(ip, "127.0.0.1");
        assert_eq!(mask, "255.255.0.0");

        let (input, (ip, mask)) = ip_mask("127.0.0.1~255.255.0.0").unwrap();
        assert_eq!(input, "");
        assert_eq!(ip, "127.0.0.1");
        assert_eq!(mask, "255.255.0.0");
        Ok(())
    }
}