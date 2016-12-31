extern crate hoodlum;

use hoodlum::*;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn compile_blinky() {
    let mut f = File::open(&"examples/blinky/blinky.hdl").unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);
    compile(&code);
}

#[test]
fn compile_ethernet() {
    let mut f = File::open(&"examples/ethernet/ethernet.hdl").unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);
    compile(&code);
}

#[test]
fn compile_ntsc() {
    let mut f = File::open(&"examples/ntsc/ntsc.hdl").unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);
    compile(&code);
}

#[test]
fn compile_sequence() {
    let mut f = File::open(&"examples/sequence/sequence.hdl").unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);
    compile(&code);
}
