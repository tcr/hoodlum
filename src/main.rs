#[macro_use] extern crate hoodlum;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use hoodlum::*;
use regex::Regex;

fn main() {
    if env::args().len() < 3 {
        println!("Usage: hoodlum input.hdl output.v");
        return;
    }

    let args = env::args().collect::<Vec<_>>();

    let mut f = File::open(&args[1]).unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);

    let re = Regex::new(r"(?m)//.*").unwrap();
    let code = re.replace_all(&code, "");

    let code = hoodlum::parse_results(&code, hoodlum::hdl_parser::parse_Code(&code));

    let verilog = code.to_verilog(&Default::default());
    println!("Verilog:");
    codelist(&verilog);

    let mut f = File::create(&args[2]).unwrap();
    f.write_all(verilog.as_bytes()).unwrap();

    println!("");
    println!("File written as {}", &args[2]);
}
