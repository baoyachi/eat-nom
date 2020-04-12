use std::net::IpAddr;
use nom::character::complete::alpha0;
use nom::error::ErrorKind;
use crate::error::*;
use nom::InputTakeAtPosition;


pub fn ip0(input: &str) -> IResult<&str, &str> {
    let (x,out) = input.split_at_position_complete(
        |item| {
            match item as u8 {
                b'0'..=b'9' => false,
                b'A'..=b'F' | b'a'..=b'f' => false,
                b':' | b'.' | b'-' => false,
                _ => true,
            }
        }
    )?;
    Ok(("",""))
}

pub fn ip1(input: &str) -> IResult<&str, IpAddr> {
    let (x,out) = input.split_at_position1_complete(
        |item| {
            match item as u8 {
                b'0'..=b'9' => false,
                b'A'..=b'F' | b'a'..=b'f' => false,
                b':' | b'.' | b'-' => false,
                _ => true,
            }
        },
        ErrorKind::Char
    )?;
    let ip = out.parse().unwrap();
    Ok((x,ip))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip() {
        let input = "@127.0.0.1!";
        println!("{:?}", ip0(input));

        let input = "127.0.0.1!";
        println!("{:?}", ip1(input));
    }
}