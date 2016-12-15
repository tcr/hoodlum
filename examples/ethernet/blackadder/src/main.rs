extern crate regex;
#[macro_use] extern crate maplit;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use regex::Regex;

struct Output {
    builtins: Vec<String>,
    fns: Vec<String>,
    buffer: Vec<u16>,
}

impl Output {
    fn builtin(&mut self, arg: u16) {
        self.buffer.push(0b1000000000000000 |
            (arg & 0b111111111111111));
    }

    fn user(&mut self, arg: u16) {
        self.buffer.push(0b1100000000000000 |
            (arg & 0b111111111111111));
    }

    fn literal(&mut self, value: i16) {
        self.buffer.push(value.abs() as u16);
        if value < 0 {
            // invert
            self.buffer.push(0b100000000000000);
        }
    }

    fn call(&mut self, arg: &str) {
        match (
            self.builtins.iter().position(|x| x == arg),
            self.fns.iter().position(|x| x == arg)
        ) {
            (Some(pos), _) => {
                self.builtin(pos as u16);
            }
            (_, Some(pos)) => {
                self.user(pos as u16);
            }
            _ => {
                panic!("Could not find arg {:?}", arg)
            }
        }
    }

    fn pos(&self) -> u16 {
        self.buffer.len() as u16
    }
}

fn split_terms(input: &str) -> Vec<String> {
    let re_term = Regex::new(r"\S+").unwrap();
    re_term.captures_iter(input).map(|x| x.at(0).unwrap().to_string()).collect()
}

fn main() {
    if env::args().len() < 2 {
        println!("Usage: blackadder input.ba");
        return;
    }

    let args = env::args().collect::<Vec<_>>();

    let mut f = File::open(&args[1]).unwrap();
    let mut code = String::new();
    let _ = f.read_to_string(&mut code);

    let mut builtin_fn = vec![
        "invert".to_string(),
        "goto".to_string(),
        "and".to_string(),
        "or".to_string(),
        "add".to_string(),
        "sub".to_string(),
        "pop".to_string(),
        "ifz".to_string(),
        "ifnz".to_string(),
        "load".to_string(),
        "store".to_string(),
    ];
    let mut state_fn = vec![];
    let mut state_mem = vec![];
    let mut state_define = hashmap![];

    // Read all decls.
    let re = Regex::new(r"(?m)#(\S+)(.*)$").unwrap();
    for decl in re.captures_iter(&code) {
        let kind = decl.at(1).unwrap().to_string();
        let re_term = Regex::new(r"\S+").unwrap();
        let terms = split_terms(decl.at(2).unwrap());
        match kind.as_ref() {
            "fn" => {
                assert_eq!(terms.len(), 1, "#fn decl should have only one term");
                state_fn.push(terms[0].clone());
            }
            "mem" => {
                assert_eq!(terms.len(), 1, "#mem decl should have only one term");
                state_mem.push(terms[0].clone());
            }
            "define" => {
                assert_eq!(terms.len(), 2, "#define decl should have only two terms");
                state_define.insert(terms[0].clone(), terms[1].clone());
            }
            _ => {
                panic!("unknown decl #{}", kind);
            }
        }
    }

    println!("fns {:?}", state_fn);
    println!("mem {:?}", state_mem);
    println!("define {:?}", state_define);

    // Strip comments.
    let re = Regex::new(r"(?m)//.*").unwrap();
    let code = re.replace_all(&code, "");

    // Strip decls.
    let re = Regex::new(r"(?m)#.*").unwrap();
    let code = re.replace_all(&code, "");

    // Parse.
    let re_label = Regex::new(r"^\.([^:]+):$").unwrap();
    let re_labelref = Regex::new(r"^\.([^:]+)$").unwrap();
    let re_num_hex = Regex::new(r"(-?)0x([a-fA-F0-9]+)").unwrap();
    let re_num_bin = Regex::new(r"(-?)0b([01]+)").unwrap();
    let re_num_dec = Regex::new(r"(-?)([0-9]+)").unwrap();
    let mut out = Output {
        buffer: vec![],
        builtins: builtin_fn.clone(),
        fns: state_fn.clone(),
    };
    let mut label_patch: Vec<(String, u16)> = vec![];
    let mut labels = hashmap![];
    for term in split_terms(&code) {
        if let Some(m) = re_label.captures(&term) {
            let label = m.at(1).unwrap().to_string();
            labels.insert(label.clone(), out.pos());
            //println!("label {:?}", label);
        } else if let Some(m) = re_labelref.captures(&term) {
            let label = m.at(1).unwrap().to_string();
            label_patch.push((label, out.pos()));
            out.literal(0);
            //println!("labelref {:?}", label);
        } else if let Some(m) = re_num_hex.captures(&term) {
            let num = m.at(0).unwrap().clone();
            let num = i16::from_str_radix(&num[2..], 16).unwrap();
            out.literal(num);
            //println!("num hex {:?}", term);
        } else if let Some(m) = re_num_bin.captures(&term) {
            let num = m.at(0).unwrap().clone();
            let num = i16::from_str_radix(&num[2..], 2).unwrap();
            out.literal(num);
            //println!("num bin {:?}", term);
        } else if let Some(m) = re_num_dec.captures(&term) {
            let num = m.at(0).unwrap().clone();
            let num = i16::from_str_radix(&num, 10).unwrap();
            out.literal(num);
            //println!("num dec {:?}", term);
        } else {
            if state_mem.iter().find(|&x| *x == term).is_some() {
                //println!("mem {:?}", term);
                out.literal(state_mem.iter().position(|x| *x == term).unwrap() as i16);
            } else if state_fn.iter().find(|&x| *x == term).is_some() || builtin_fn.iter().find(|&x| *x == term).is_some() {
                //println!("fn {:?}", term);
                out.call(&term);
            } else if state_define.iter().find(|&x| *x.0 == term).is_some() {
                //println!("define {:?}", term);
                out.literal(state_define.iter().position(|x| *x.0 == term).unwrap() as i16);
            } else {
                panic!("unknown term {:?}", term);
            }
        }
    }

    for (name, pos) in label_patch {
        let label_pos = labels.iter().position(|x| *x.0 == name).expect(&format!("Could not find label ref {:?}", name));
        out.buffer[pos as usize] = label_pos as u16;
    }

    // Print.
    println!("");
    println!("");
    println!("");
    println!("const BINARY_LEN = {};", out.buffer.len());
    println!("def binary: bit[16][{}] = {{", out.buffer.len());
    for item in out.buffer {
        println!("    16b{},", format!("{:016b}", item)
            .chars().map(|x| x.to_string()).collect::<Vec<String>>()
            .chunks(4).map(|x| x.join("")).collect::<Vec<String>>()
            .join("_"));
    }
    println!("}};")
    //println!("{}", code);
}
