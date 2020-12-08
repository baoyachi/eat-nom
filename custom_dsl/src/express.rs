use nom::sequence::tuple;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use crate::kv::kvs;
use crate::extra::extra;
use nom::combinator::opt;

fn express(input: &str) -> nom::IResult<&str, (Vec<(&str, &str)>, Vec<(&str, Vec<&str>)>)> {
    let (input, (_, _, kvs, _, extra, _, _)) = tuple((
        tag("{"),
        multispace0,
        kvs,
        multispace0,
        opt(extra),
        multispace0,
        tag("}"),
    ))(input)?;
    let mut value = vec![];
    if let Some(ext) = extra {
        value = ext;
    }
    Ok((input, (kvs, value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_express() {
        let input = r#"{a|b|c:x a="x" b="y" c=["z1","z2","z3"]}~rust"#;
        let (input, out) = express(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!((
                       vec![
                           ("a", "x"),
                           ("b", "x"),
                           ("c", "x")
                       ],
                       vec![
                           ("a", vec!["x"]),
                           ("b", vec!["y"]),
                           ("c", vec!["z1", "z2", "z3"]),
                       ],
                   ), out);
    }

    #[test]
    fn test_express_1() {
        let input = r#"{a:a|b:b|c:c a="x" b="y" c=["z1","z2","z3"]}~rust"#;
        let (input, out) = express(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!((
                       vec![
                           ("a", "a"),
                           ("b", "b"),
                           ("c", "c")
                       ],
                       vec![
                           ("a", vec!["x"]),
                           ("b", vec!["y"]),
                           ("c", vec!["z1", "z2", "z3"]),
                       ],
                   ), out);
    }

    #[test]
    fn test_express_opt() {
        let input = r#"{a|b|c:x}@rust"#;
        let (input, out) = express(input).unwrap();
        assert_eq!("@rust", input);
        assert_eq!((
                       vec![
                           ("a", "x"),
                           ("b", "x"),
                           ("c", "x")
                       ],
                       vec![],
                   ), out);
    }
}