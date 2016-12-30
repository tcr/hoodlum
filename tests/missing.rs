extern crate hoodlum;

use hoodlum::*;

#[test]
fn succeed_simple() {
    let code = r#"
entity Main {
    in clk: bit,
    out LED1: bit,
}

impl Main {
    on clk.posedge {
        LED1 <= !LED1;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_Code(code));
    typecheck(&res);
}

#[test]
#[should_panic]
fn missing_def() {
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

#[test]
fn def_in_other_module() {
    let code = r#"
entity Other {
    in index: bit,
}

impl Other {
    index = 1;
}

entity Main {
    in clk: bit,
    out LED1: bit,
}

impl Main {
    on clk.posedge {
        LED1 <= !LED1;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_Code(code));
    typecheck(&res);
}
