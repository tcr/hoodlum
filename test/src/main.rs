#[macro_use] extern crate hoodlum;

use std::io::prelude::*;
use std::fs::File;
use hoodlum::*;

fn main() {
    let code = hdl! {

module (clk: in, LED1: out, LED2: out, LED3: out, LED4: out, LED5: out) {
    reg rot[4] = 0b0001;
    reg ready = 1;
    reg divider[21] = 0;

    on clk.posedge {
        reset ready {
            if divider == 1200000 - 1 {
                divider <= 0;
                rot <= {rot[3-1:0], rot[3]};
            } else {
                divider <= divider + 1;
            }
        }
    }

    always {
        LED1 = rot[0];
        LED2 = rot[1];
        LED3 = rot[2];
        LED4 = rot[3];
        LED5 = 1;
    }
}

};

    println!("");
    println!("AST: {:?}", code);

    println!("");
    let verilog = code.to_verilog(&Default::default());
    println!("Verilog:\n{}", verilog);

    let mut f = File::create("out/test.v").unwrap();
    f.write_all(verilog.as_bytes()).unwrap();

    println!("");
    println!("File written as out/test.v");
}
