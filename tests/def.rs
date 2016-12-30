extern crate hoodlum;

use hoodlum::*;

#[test]
fn def_always() {
    let code = r#"
def a = b | c;
"#;

    let valid = r#"
reg [(1)-1:0] a;
always @(*) a = (b | c);
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_Decl(code));
    let out = res.to_verilog(&Default::default());
    assert_eq!(out.trim(), valid.trim());
}


#[test]
#[should_panic]
fn def_mut_with_assignment() {
    let code = r#"
def mut a = b | c;
"#;

    parse_results(code, hoodlum::hdl_parser::parse_Decl(code));
}
