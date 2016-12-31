#[macro_use] extern crate hoodlum;
extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use hoodlum::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Hoodlum")
                      .version("0.4.0")
                    //  .arg(Arg::with_name("version")
                    //       .short("V")
                    //       .long("version")
                    //       .help("Reads the current version")
                    //       .takes_value(false))
                      .arg(Arg::with_name("INPUT")
                           .help("Sets the input Hoodlum file to use")
                           .required(true)
                           .index(1))
                       .arg(Arg::with_name("OUTPUT")
                            .short("o")
                            .long("output")
                            .help("Sets the Verilog output filename.")
                            .takes_value(true))
                      .get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT");

    let mut f = File::open(&input_file).unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);

    // Compile file to code.
    let verilog = compile(&code);
    println!("Verilog:");
    codelist(&verilog);

    if let Some(output_file) = output_file {
        let mut f = File::create(&output_file).unwrap();
        f.write_all(verilog.as_bytes()).unwrap();
        println!("");
        println!("File written as {}", &output_file);
    }
}
