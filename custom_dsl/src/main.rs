mod express;
mod extra;
mod key;
mod kv;
use express::express;

fn main() {
    // let extra_basic = r#"a="x""#;
    // let extra_kv_obj = r#"a="x" b="y" c="z1 z2""#;
    // let extra_kv_arr = r#"s1=["a","b","c"] s2=["x","y","z"]"#;
    // let extra_mix_obj_arr = r#"s1=["a","b","c"] s2=["x","y","z"] s3="s""#;
    //
    //
    // let input_basic = "a:x";
    // let input_kv = "a:x|b:y|c:z";
    // let input_or_kv = "a|b|c:x";
    //
    //
    // let express_0 = r#"{a:x|b:y|c:z}"#;
    // let express_1 = r#"{a|b|c:x}"#;
    // let express_2 = r#"{a|b|c:x e="e1"}"#;
    // let express_3 = r#"{a|b|c:x e="e1" f="f1"}"#;
    // let express_3 = r#"{a:x|b:y|c:z e="e1" f="f1"}"#;
    //
    // let express_5 = r#"{a:a|b:b|c:c d="d1" e=["e1","e2"]}"#;

    let input = r#"{a|b|c:x d="x" e="y" f=["z1","z2","z3"]}~hello rust"#;
    let (input, out) = express(input).unwrap();
    assert_eq!("~hello rust", input);
    assert_eq!(
        (
            vec![("a", "x"), ("b", "x"), ("c", "x")],
            vec![
                ("d", vec!["x"]),
                ("e", vec!["y"]),
                ("f", vec!["z1", "z2", "z3"]),
            ],
        ),
        out
    );
}
