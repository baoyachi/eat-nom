use crate::key::key;
use nom::character::complete::{multispace0, multispace1};
use nom::bytes::complete::{tag, take_until};
use nom::sequence::tuple;
use nom::multi::separated_list0;
use nom::combinator::opt;


/// a="x"
/// a = "x"
fn extra_obj(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let (input, (k, _, _, _, _, v, _)) = tuple((key, multispace0, tag("="), multispace0, tag("\""), take_until("\""), tag("\"")))(input)?;
    Ok((input, (k, v)))
}

/// a = ["x","y","z"]
fn sep_arr(input: &str) -> nom::IResult<&str, Vec<&str>> {
    let mut value = vec![];
    let (input, val) = separated_list0(
        tag(","),
        tuple((
            multispace0,
            tag("\""),
            take_until("\""),
            tag("\""),
            multispace0,
        )),
    )(input)?;
    for (_, _, v, _, _) in val {
        value.push(v);
    }
    Ok((input, value))
}


/// a = ["x","y","z"]
fn extra_arr(input: &str) -> nom::IResult<&str, (&str, Vec<&str>)> {
    let (input, (k, _, _, _, _, v, _)) = tuple((
        key,
        multispace0,
        tag("="),
        multispace0,
        tag("["),
        sep_arr,
        tag("]")
    ))(input)?;
    Ok((input, (k, v)))
}

///a= "x" b=["x","y","z"]
pub fn extra(input: &str) -> nom::IResult<&str, Vec<(&str, Vec<&str>)>> {
    let (input, val) = separated_list0(
        multispace1,
        tuple((opt(extra_obj), opt(extra_arr))),
    )(input)?;

    //out:[(Some(("a", "x")), None), (None, Some(("b", ["x", "y", "z"])))]
    let mut k_vec = vec![];
    for (obj, arr) in val {
        if let Some((k, v)) = obj {
            k_vec.push((k, vec![v]))
        }
        if let Some((k, vs)) = arr {
            let mut v_vec = vec![];
            for v in vs {
                v_vec.push(v);
            }
            k_vec.push((k, v_vec));
        }
    }
    Ok((input, k_vec))
}


#[cfg(test)]
mod tests {
    use super::*;

    // let extra_basic = r#"a="x""#;
    // let extra_arr = r#"a=["x","y","z"]";
    // let extra_kv_obj = r#"a="x" b="y" c="z1 z2""#;
    // let extra_kv_arr = r#"s1=["a","b","c"] s2=["x","y","z"]"#;
    // let extra_mix_obj_arr = r#"s1=["a","b","c"] s2=["x","y","z"] s3="s""#;
    #[test]
    fn test_extra() {
        let input = r#"a= "x" b=["x","y","z"]~rust"#;
        let (input, out) = extra(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", vec!["x"]),
            ("b", vec!["x", "y", "z"])
        ], out);
    }


    #[test]
    fn test_extra_1() {
        let input = r#"a= "x" b=["x","y","z"] c="c" d=["d1","d2", "d3"]~rust"#;
        let (input, out) = extra(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!(vec![
            ("a", vec!["x"]),
            ("b", vec!["x", "y", "z"]),
            ("c", vec!["c"]),
            ("d", vec!["d1", "d2", "d3"])
        ], out);
    }


    #[test]
    fn test_extra_arr() {
        let input = r#"a=["x","y","z"] rust"#;
        let (input, (k, v)) = extra_arr(input).unwrap();
        assert_eq!(" rust", input);
        assert_eq!("a", k);
        assert_eq!(vec!["x", "y", "z"], v);
    }

    #[test]
    fn test_extra_arr_2() {
        let input = r#"a = [ "x" , "y", "z"]~rust"#;
        let (input, (k, v)) = extra_arr(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!("a", k);
        assert_eq!(vec!["x", "y", "z"], v);
    }

    #[test]
    fn test_sep_arr1() {
        let input = r#""x","y","z" rust"#;
        let (input, out) = sep_arr(input).unwrap();
        assert_eq!("rust", input);
        assert_eq!(out, vec!["x", "y", "z"]);
    }

    #[test]
    fn test_sep_arr2() {
        let input = r#""x","y",   "z" rust"#;
        let (input, out) = sep_arr(input).unwrap();
        assert_eq!("rust", input);
        assert_eq!(out, vec!["x", "y", "z"]);
    }


    #[test]
    fn test_extra_basic_1() {
        let input = r#"a="x" ok"#;
        let (input, (k, v)) = extra_obj(input).unwrap();
        assert_eq!(" ok", input);
        assert_eq!("a", k);
        assert_eq!("x", v);
    }

    #[test]
    fn test_extra_basic_2() {
        let input = r#"a = "x"~rust"#;
        let (input, (k, v)) = extra_obj(input).unwrap();
        assert_eq!("~rust", input);
        assert_eq!("a", k);
        assert_eq!("x", v);
    }
}