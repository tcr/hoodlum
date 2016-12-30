extern crate hoodlum;

use hoodlum::*;

#[test]
fn missing_1() {
    let code = r#"
entity Main {
    in clk: bit,
    out LED1: bit,
}

impl Main {
    on clock.posedge {
        LED1 <= !LED1;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_Code(code));

    typecheck(&res);
}
