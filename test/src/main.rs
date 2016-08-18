#[macro_use] extern crate hoodlum;

use std::io::prelude::*;
use std::fs::File;
use hoodlum::*;

fn main() {
    let code = hdl! {

module (clk: in, LED1: out, LED2: out, LED3: out, LED4: out, LED5: out) {
    reg rot[4] = 0b0001;
    reg ready = 0;
    reg divider[0..1200000] = 0;

    reg index[0..1200000] = 0;

    reg state[0..3] = 0;

    on clk.posedge {
        reset ready {
            if divider == 1200000 - 1 {
                divider <= 0;
                match state {
                    0 => {
                        rot <= 0b0001;
                        state <= 1;
                    }
                    1 => {
                        rot <= 0b0010;
                        state <= 2;
                    }
                    2 => {
                        rot <= 0b0100;
                        state <= 3;
                    }
                    3 => {
                        rot <= 0b1000;
                        state <= 0;
                    }
                }
            } else {
                divider <= divider + 1;
            }
        }
    }

    on clk.posedge {
        if index == 12000000 - 1 {
            ready <= 1;
            LED5 <= 0;
        } else {
            index <= index + 1;
            LED5 <= 1;
        }
    }

    always {
        LED1 = ready && rot[0];
        LED2 = ready && rot[1];
        LED3 = ready && rot[2];
        LED4 = ready && rot[3];
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
