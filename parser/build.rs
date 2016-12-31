extern crate libflate;

use std::io;
use std::fs::File;
use std::path::Path;
use std::env;
use libflate::gzip::Decoder;

fn main() {
    let out_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    let mut input = File::open(out_dir.join("src/hdl_parser.rs.gz")).unwrap();
    let mut output = File::create(out_dir.join("src/hdl_parser.rs")).unwrap();
    let mut decoder = Decoder::new(&mut input).unwrap();
    io::copy(&mut decoder, &mut output).unwrap();
}
