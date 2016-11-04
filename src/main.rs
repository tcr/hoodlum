#[macro_use] extern crate hoodlum;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use hoodlum::*;

fn main() {
    if env::args().len() < 2 {
        println!("Usage: hoodlum input.hdl output.v");
        return;
    }

    let mut f = File::open(env::args().next().unwrap()).unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);

    let code = hoodlum::parse_results(&code, hoodlum::hdl_parser::parse_Code(&code));

    let verilog = code.to_verilog(&Default::default());
    println!("Verilog:");
    codelist(&verilog);

    let mut f = File::create("out/test.v").unwrap();
    f.write_all(verilog.as_bytes()).unwrap();

    println!("");
    println!("File written as out/test.v");
}
