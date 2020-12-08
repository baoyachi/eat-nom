use nom::sequence::{tuple, delimited};
use crate::key::key;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::separated_list0;

///a:x
fn kv(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let (input, (k, _, _, _, v, _)) = tuple((
        key,
        multispace0,
        tag(":"),
        multispace0,
        key,
        multispace0,
    ))(input)?;
    Ok((input, (k, v)))
}

// a:x|b:y|c:z
fn kv_multi(input: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    let (input, out) = separated_list0(
        tag("|"),
        kv,
    )(input)?;
    Ok((input, out))
}

/// a|b|c:x
fn sep_kv_multi(input: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    let (input, (_, val, _, _, _, v)) = tuple((
        multispace0,
        separated_list0(tag("|"), delimited(multispace0, key, multispace0)),
        multispace0,
        tag(":"),
        multispace0,
        key,
    ))(input)?;
    let mut value = vec![];
    for k in val {
        value.push((k, v))
    }
    Ok((input, value))
}

pub fn kvs(input: &str) -> nom::IResult<&str, Vec<(&str, &str)>> {
    let (input, out) = kv_multi(input)?;
    if !out.is_empty() {
        return Ok((input, out));
    }
    return sep_kv_multi(input);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kvs() {
        let input = r#"a|b|c:x~rust"#;
        let (input, out) = kvs(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", "x"),
            ("b", "x"),
            ("c", "x"),
        ], out);
    }

    #[test]
    fn test_kvs_1() {
        let input = r#"a:x|b:y|c:z~rust"#;
        let (input, out) = kvs(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", "x"),
            ("b", "y"),
            ("c", "z"),
        ], out);
    }

    #[test]
    fn test_sep_kv_multi() {
        let input = r#"a|b|c:x~rust"#;
        let (input, out) = sep_kv_multi(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", "x"),
            ("b", "x"),
            ("c", "x"),
        ], out);
    }

    #[test]
    fn test_sep_kv_multi_2() {
        let input = r#"a | b | c:x~rust"#;
        let (input, out) = sep_kv_multi(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", "x"),
            ("b", "x"),
            ("c", "x"),
        ], out);
    }


    #[test]
    fn test_kv_multi() {
        let input = r#"a:x|b:y~rust"#;
        let (input, out) = kv_multi(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", "x"),
            ("b", "y"),
        ], out);
    }

    #[test]
    fn test_kv() {
        let input = r#"a:x~rust"#;
        let (input, (k, v)) = kv(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!("a", k);
        assert_eq!("x", v);
    }
}