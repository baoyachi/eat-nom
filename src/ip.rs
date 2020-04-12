use std::net::IpAddr;
use nom::character::complete::alpha0;
use nom::error::ErrorKind;
use crate::error::*;
use nom::InputTakeAtPosition;

pub fn ip(input: &str) -> IResult<&str, IpAddr> {
    let (input, out) = input.split_at_position1_complete(
        |item| {
            match item as u8 {
                b'0'..=b'9' => false,
                b'A'..=b'F' | b'a'..=b'f' => false,
                b':' | b'.' | b'-' => false,
                _ => true,
            }
        },
        ErrorKind::Char,
    )?;
    let ip = out.parse::<IpAddr>()?;
    Ok((input, ip))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip() -> Result<()>{
        let input = "127.0.0.1";
        let (input,ip) = ip(input)?;
        assert_eq!(input,"");
        assert_eq!(ip,"127.0.0.1".parse::<IpAddr>()?);
        Ok(())
    }
}