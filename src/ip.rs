use crate::error::EResult;
use nom::{
    branch::alt, bytes::complete::tag, error::ErrorKind, sequence::tuple, InputTakeAtPosition,
};
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

pub fn parse_ip(input: &str) -> EResult<(&str, IpAddr)> {
    let (input, out) = ip(input)?;
    let ip = out.parse::<IpAddr>()?;
    Ok((input, ip))
}

pub fn parse_ip_mask<'a>(input: &'a str, concat: &'a str) -> EResult<(&'a str, (IpAddr, IpAddr))> {
    let (input, (ip, _, mask)) = tuple((ip, tag(concat), mask))(input)?;
    let ip = ip.parse::<IpAddr>()?;
    let mask = mask.parse::<IpAddr>()?;
    Ok((input, (ip, mask)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_ip_mask() -> EResult<()> {
        let ip_mask = "127.0.0.1/255.0.255.0";
        let (input, (ip, mask)) = parse_ip_mask(ip_mask, "/")?;
        assert_eq!(input, "");
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(mask, Ipv4Addr::new(255, 0, 255, 0));
        Ok(())
    }

    #[test]
    fn test_parse_ip() -> EResult<()> {
        let ipv4 = "127.0.0.1";
        let (input, ip) = parse_ip(ipv4)?;
        assert_eq!(input, "");
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        Ok(())
    }
}
