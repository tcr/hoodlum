extern crate hoodlum;

use hoodlum::*;

//TODO
#[test]
#[ignore]
fn match_or() {
    let code = r#"
match a {
    0 | 1 => { }
    _ => { }
}
"#;

    let valid = r#"
case (a)
    0, 1: begin
    end
    default: begin
    end
endcase
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));
    let out = res.to_verilog(&Default::default());
    assert_eq!(out.trim(), valid.trim());
}

//TODO
#[ignore]
#[test]
#[should_panic]
fn match_without_default() {
    let code = r#"
match a {
    0 => { }
    1 => { }
}
"#;

    let _ = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));
}

//TODO
#[ignore]
#[test]
fn match_expr() {
    let code = r#"
match a {
    0 => a <= 1,
    1 => a <= 2,
    2 => a <= 0,
}
"#;

    let valid = r#"
case (a)
    0, 1: begin
    end
    default: begin
    end
endcase
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));
    let out = res.to_verilog(&Default::default());
    assert_eq!(out.trim(), valid.trim());
}
