//! # Hoodlum
//! Is the coolest.

#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate matches;
extern crate hoodlum_parser;
extern crate lalrpop_util;
extern crate regex;

pub mod sequence;
pub mod verilog;
pub mod walker;

pub use hoodlum_parser::{ParseError, ast, hdl_parser};
pub use verilog::ToVerilog;
pub use walker::*;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt::Debug;
use regex::Regex;

pub fn codelist(code: &str) {
    for (i, line) in code.lines().enumerate() {
        println!("{:>3} | {}", i+1, line);
    }
}

pub fn code_error(code: &str, tok_pos: usize) {
    let code = format!("\n\n{}", code);
    let code = code.lines().collect::<Vec<_>>();
    let mut pos: isize = 0;
    for (i, lines) in (&code[..]).windows(3).enumerate() {
        if pos + lines[2].len() as isize >= tok_pos as isize {
            if i > 1 {
                println!("{:>3} | {}", i - 1, lines[0]);
            }
            if i > 0 {
                println!("{:>3} | {}", i, lines[1]);
            }
            println!("{:>3} | {}", i + 1, lines[2]);

            println!("{}^", (0..(tok_pos as isize) - (pos - 6)).map(|_| "~").collect::<String>());
            return;
        }
        pos += (lines[2].len() as isize) + 1;
    }
}

pub fn parse_results<C,T,E>(code: &str, res: Result<C, ParseError<usize,T,E>>) -> C
where C: Debug, T: Debug, E: Debug {
    match res {
        Ok(value) => {
            return value;
        }
        Err(ParseError::InvalidToken {
            location: loc
        }) => {
            println!("Error: Invalid token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        Err(ParseError::UnrecognizedToken {
            token: Some((loc, _, _)),
            ..
        }) => {
            println!("Error: Unrecognized token:");
            code_error(code, loc);
            panic!("{:?}", res);
        }
        err => {
            panic!("{:?}", err);
        }
    }
}

#[macro_export]
macro_rules! hdl {
    ( $( $x:tt )* ) => {
        {
            let code = stringify!($($x)*);

            println!("Input");
            codelist(code);
            println!("");
            hoodlum::parse_results(&code, hoodlum::hdl_parser::parse_Code(&code))
        }
    };
}

pub struct TypeCollector {
    inner_types: BTreeMap<String, (Option<Vec<ast::Arg>>, Option<Vec<ast::Decl>>)>,
    literals: Vec<String>,
}

impl TypeCollector {
    pub fn new() -> TypeCollector {
        TypeCollector {
            inner_types: btreemap![],
            literals: vec![],
        }
    }

    // TODO make result
    pub fn validate(&self) {
        for (key, &(ref args, _)) in &self.inner_types {
            if args.is_none() {
                panic!("Declared {:?} without entity definition", key);
            }
        }
    }

    pub fn types(&self) -> BTreeMap<String, (Vec<ast::Arg>, Option<Vec<ast::Decl>>)> {
        self.inner_types.clone().into_iter().map(|x| {
            (x.0, ((x.1).0.unwrap(), (x.1).1))
        }).collect::<BTreeMap<_, _>>()
    }
}

impl Walker for TypeCollector {
    fn toplevel(&mut self, top: &ast::Toplevel) {
        match top {
            &ast::Toplevel::Entity(ref id, ref args) => {
                match self.inner_types.entry(id.0.clone()) {
                    Entry::Vacant(vacant) => {
                        vacant.insert((Some(args.clone()), None));
                    }
                    Entry::Occupied(mut occupied) => {
                        match occupied.get_mut() {
                            &mut (ref mut ref_args @ None, _) => {
                                *ref_args = Some(args.clone());
                            }
                            &mut (Some(_), _) => {
                                panic!("Repeated entity definition of {:?}", id.0);
                            }
                        }
                    }
                }
            }
            &ast::Toplevel::Impl(ref id, ref args) => {
                match self.inner_types.entry(id.0.clone()) {
                    Entry::Vacant(vacant) => {
                        vacant.insert((None, Some(args.clone())));
                    }
                    Entry::Occupied(mut occupied) => {
                        match occupied.get_mut() {
                            &mut (_, ref mut ref_args @ None) => {
                                *ref_args = Some(args.clone());
                            }
                            &mut (_, Some(_)) => {
                                panic!("Repeated impl definition of {:?}", id.0);
                            }
                        }
                    }
                }
            },
            &ast::Toplevel::VerilogLiteral(ref code) => {
                self.literals.push(code.clone());
            }
        }
    }
}

pub struct RefChecker {
    pub valid: Vec<String>,
}

impl RefChecker {
    pub fn new() -> Self {
        RefChecker {
            valid: vec![],
        }
    }
}

impl Walker for RefChecker {
    fn expr(&mut self, expr: &ast::Expr) {
        match expr {
            &ast::Expr::Ref(ref id) => {
                let id_str = &id.0;
                if self.valid.iter().position(|x| *x == *id_str).is_none() {
                    panic!("Invalid reference to undefined var {:?}", id_str);
                }
            }
            &ast::Expr::Slice(ref id, _, _) => {
                let id_str = &id.0;
                if self.valid.iter().position(|x| *x == *id_str).is_none() {
                    panic!("Invalid slice reference to undefined var {:?}", id_str);
                }
            }
            _ => { }
        }
    }

    fn edgeref(&mut self, expr: &ast::EdgeRef) {
        let id_str = &(expr.0).0;
        if self.valid.iter().position(|x| *x == *id_str).is_none() {
            panic!("Invalid clock reference to undefined var {:?}", id_str);
        }
    }

    fn seq(&mut self, expr: &ast::Seq) {
        match expr {
            &ast::Seq::Set(_, ref id, _) => {
                let id_str = &id.0;
                if self.valid.iter().position(|x| *x == *id_str).is_none() {
                    panic!("Invalid assignment to undefined var {:?}", id_str);
                }
            }
            _ => { }
        }
    }
}


pub struct DefMutChecker {
    pub valid: Vec<String>,
}

impl DefMutChecker {
    pub fn new() -> Self {
        DefMutChecker {
            valid: vec![],
        }
    }
}

impl Walker for DefMutChecker {
    fn seq(&mut self, expr: &ast::Seq) {
        match expr {
            &ast::Seq::Set(ast::BlockType::Blocking, ref id, _) |
            &ast::Seq::Set(ast::BlockType::NonBlocking, ref id, _) => {
                let id_str = &id.0;
                if self.valid.iter().position(|x| *x == *id_str).is_none() {
                    panic!("Invalid assignment to non-mutable var {:?}", id_str);
                }
            }
            _ => { }
        }
    }
}


pub struct DefChecker {
    pub valid: Vec<String>,
}

impl DefChecker {
    pub fn new() -> Self {
        DefChecker {
            valid: vec![],
        }
    }
}

impl Walker for DefChecker {
    fn seq(&mut self, expr: &ast::Seq) {
        match expr {
            &ast::Seq::Set(ast::BlockType::Static, ref id, _) => {
                let id_str = &id.0;
                if self.valid.iter().position(|x| *x == *id_str).is_none() {
                    panic!("Invalid static assignment to mutable var {:?}", id_str);
                }
            }
            _ => { }
        }
    }
}

//TODO not abort
pub fn typecheck(code: &ast::Code) {
    let mut types = TypeCollector::new();
    code.walk(&mut types);
    types.validate();

    for (_, entity) in types.types() {
        // Extract consts.
        let mut inner_consts = vec![];
        for item in entity.1.clone().unwrap_or(vec![]) {
            //TODO if shadow an entity arg, panic
            match item {
                ast::Decl::Const(id, _) => {
                    inner_consts.push(id.0.clone());
                }
                _ => { }
            }
        }

        // Extract defs.
        let mut inner_defs = vec![];
        for item in entity.1.clone().unwrap_or(vec![]) {
            //TODO if shadow an entity arg, panic
            match item {
                ast::Decl::Let(id, _, _) => {
                    inner_defs.push(id.0.clone());
                    //println!("obj def {:?}", id);
                }
                ast::Decl::Reg(id, _, _) => {
                    inner_defs.push(id.0.clone());
                    //println!("reg {:?}", id);
                }
                ast::Decl::Const(id, _) => {
                    inner_consts.push(id.0.clone());
                }
                _ => { }
            }
        }

        // Extract def muts.
        let mut inner_def_muts = vec![];
        for item in entity.1.clone().unwrap_or(vec![]) {
            //TODO if shadow an entity arg, panic
            match item {
                ast::Decl::Latch(id, _) => {
                    inner_def_muts.push(id.0.clone());
                    //println!("latch {:?}", id);
                }
                _ => { }
            }
        }

        // Check that all inner refs are declared.
        let mut checker = RefChecker::new();
        checker.valid.extend(entity.0.clone().iter().map(|x| (x.0).0.clone()));
        checker.valid.extend(inner_consts.clone());
        checker.valid.extend(inner_defs.clone());
        checker.valid.extend(inner_def_muts.clone());
        for decl in entity.1.clone().unwrap_or(vec![]) {
            decl.walk(&mut checker);
        }

        // Check that all static assignments are non-mutable.
        //TODO only include non-mutable arguments
        let mut checker = DefChecker::new();
        checker.valid.extend(entity.0.clone().iter().map(|x| (x.0).0.clone()));
        checker.valid.extend(inner_consts.clone());
        checker.valid.extend(inner_defs.clone());
        for decl in entity.1.clone().unwrap_or(vec![]) {
            decl.walk(&mut checker);
        }

        // Check that all latch assignments are mutable.
        //TODO only include mutable arguments
        let mut checker = DefMutChecker::new();
        checker.valid.extend(entity.0.clone().iter().map(|x| (x.0).0.clone()));
        checker.valid.extend(inner_consts.clone());
        checker.valid.extend(inner_def_muts.clone());
        for decl in entity.1.clone().unwrap_or(vec![]) {
            decl.walk(&mut checker);
        }
    }

    // TODO iterate through code, identify type decls. Then check AST for
    // incorrect references.
    //panic!("okay");
}

// Easy invocation of Verilog compilation.
pub fn compile(code: &str) -> String {
    // Removes comments.
    let re = Regex::new(r"(?m)//.*").unwrap();
    let code = re.replace_all(&code, "");

    let code = parse_results(&code, hdl_parser::parse_Code(&code));
    typecheck(&code);

    // Collect into types list.
    let mut types = TypeCollector::new();
    code.walk(&mut types);
    types.validate();

    // Convert typeset to code.
    types.to_verilog(&Default::default())
}
