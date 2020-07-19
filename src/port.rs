use nom::character::complete::digit1;
use crate::error::EatResult;
use std::ops::Range;
use nom::sequence::tuple;
use nom::branch::alt;
use nom::bytes::complete::tag;
use crate::util::new_range;


pub fn parse_port(input: &str) -> EatResult<usize> {
    let (_, digit) = digit1(input)?;
    let digit = digit.parse::<usize>()?;
    Ok(digit)
}

pub fn parse_port_range_opt(input: &str) -> EatResult<Range<usize>> {
    let (_, (start, _, end)) = tuple((digit1,
                                      alt((tag("/"), tag("-"), tag(" "), tag("\\"))),
                                      digit1))(input)?;
    let start = start.parse::<usize>()?;
    let end = end.parse::<usize>()?;
    Ok((new_range(start, end)))
}

pub fn parse_port_range<'a>(input: &'a str, concat: &'a str) -> EatResult<Range<usize>> {
    let (_, (start, _, end)) = tuple((digit1,
                                      tag(concat),
                                      digit1))(input)?;
    let start = start.parse::<usize>()?;
    let end = end.parse::<usize>()?;
    Ok(new_range(start, end))
}