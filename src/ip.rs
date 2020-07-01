use nom::{
    branch::alt,
    bytes::complete::tag,
    error::ErrorKind,
    sequence::tuple,
    InputTakeAtPosition,
};
use crate::error::EResult;
use std::net::IpAddr;

pub fn ip(input: &str) -> nom::IResult<&str, &str> {
    input.split_at_position1_complete(
        |item| match item as u8 {
            b'0'..=b'9' => false,
            b'A'..=b'F' | b'a'..=b'f' => false,
            b':' | b'.' => false,
            _ => true,
        },
        ErrorKind::Char,
    )
}

pub fn mask(input: &str) -> nom::IResult<&str, &str> {
    ip(input)
}

pub fn ip_mask(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let split = alt((tag("/"), tag("-"), tag(" "), tag("~")));
    let (input, (ip, _, mask)) = tuple((ip, split, mask))(input)?;
    Ok((input, (ip, mask)))
}

pub fn parse_ip(input: &str) -> EResult<(&str, IpAddr)> {
    let (input, out) = ip(input)?;
    let ip = out.parse::<IpAddr>()?;
    Ok((input, ip))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::error1::Result;
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_ip() -> EResult<()> {
        let ipv4 = "127.0.0.1";
        let (input, ip) = parse_ip(ipv4)?;
        assert_eq!(input, "");
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        Ok(())
    }

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
