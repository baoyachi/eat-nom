use crate::error::EatResult;
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::{bytes::complete::tag, error::ErrorKind, sequence::tuple, InputTakeAtPosition};
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

pub fn parse_ip(input: &str) -> EatResult<IpAddr> {
    let (_, out) = ip(input)?;
    let ip = out.parse::<IpAddr>()?;
    Ok(ip)
}

/// # Example
/// ```rust
///
///
/// use eat_nom::ip::parse_ip_mask;
/// use std::net::{Ipv4Addr, IpAddr};
///
/// assert_eq!(parse_ip_mask("127.0.0.1/255.0.255.0","/").unwrap(),
///           (("127.0.0.1".parse::<IpAddr>().unwrap(),"255.0.255.0".parse::<IpAddr>().unwrap(),))
/// );
///
/// ```
pub fn parse_ip_mask<'a>(input: &'a str, concat: &'a str) -> EatResult<(IpAddr, IpAddr)> {
    let (_, (ip, _, mask)) = tuple((ip, tag(concat), mask))(input)?;
    let ip = ip.parse::<IpAddr>()?;
    let mask = mask.parse::<IpAddr>()?;
    Ok((ip, mask))
}

pub fn parse_ip_mask_opt(input: &str) -> EatResult<(IpAddr, IpAddr)> {
    let (_, (ip, _, mask)) =
        tuple((ip, alt((tag("/"), tag("-"), tag(" "), tag("\\"))), mask))(input)?;
    let ip = ip.parse::<IpAddr>()?;
    let mask = mask.parse::<IpAddr>()?;
    Ok((ip, mask))
}

pub fn parse_ip_cidr<'a>(input: &'a str, concat: &'a str) -> EatResult<(IpAddr, usize)> {
    let (_, (ip, _, cidr)) = tuple((ip, tag(concat), digit1))(input)?;
    let ip = ip.parse::<IpAddr>()?;
    let cidr = cidr.parse::<usize>()?;
    Ok((ip, cidr))
}

pub fn parse_ip_cidr_opt(input: &str) -> EatResult<(IpAddr, usize)> {
    let (_, (ip, _, cidr)) =
        tuple((ip, alt((tag("/"), tag("-"), tag(" "), tag("\\"))), digit1))(input)?;
    let ip = ip.parse::<IpAddr>()?;
    let cidr = cidr.parse::<usize>()?;
    Ok((ip, cidr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_parse_ip_mask() -> EatResult<()> {
        let ip_mask = "127.0.0.1/255.0.255.0";
        let (ip, mask) = parse_ip_mask(ip_mask, "/")?;
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(mask, Ipv4Addr::new(255, 0, 255, 0));
        Ok(())
    }

    #[test]
    fn test_parse_ip_mask_opt() -> EatResult<()> {
        let ip_mask = "127.0.0.1/255.0.255.0";
        let (ip, mask) = parse_ip_mask_opt(ip_mask)?;
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(mask, Ipv4Addr::new(255, 0, 255, 0));
        Ok(())
    }

    #[test]
    fn test_parse_ip() -> EatResult<()> {
        let ipv4 = "127.0.0.1";
        let ip = parse_ip(ipv4)?;
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        Ok(())
    }

    #[test]
    fn test_parse_ip_cidr() -> EatResult<()> {
        let ip_cidr = "127.0.0.1/25";
        let (ip, cidr) = parse_ip_cidr(ip_cidr, "/")?;
        assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(cidr, 25);
        Ok(())
    }
}
