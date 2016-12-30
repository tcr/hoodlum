#[macro_use] extern crate itertools;
#[macro_use] extern crate maplit;
#[macro_use] extern crate matches;
#[macro_use] extern crate lazy_static;
extern crate hoodlum_parser;
extern crate lalrpop_util;

pub mod sequence;
pub mod verilog;

pub use hoodlum_parser::{ParseError, ast, hdl_parser};
use std::collections::HashMap;
use std::fmt::Debug;
pub use verilog::ToVerilog;

pub fn codelist(code: &str) {
    for (i, line) in code.lines().enumerate() {
        println!("{:>3} | {}", i+1, line);
    }
}

pub fn code_error(code: &str, tok: usize) {
    let code = format!("\n\n{}", code);
    let code = code.lines().collect::<Vec<_>>();
    let mut pos = 0;
    for (i, lines) in (&code[..]).windows(3).enumerate() {
        if pos + lines[2].len() >= tok {
            println!("{:>3} | {}", i - 1, lines[0]);
            println!("{:>3} | {}", i, lines[1]);
            println!("{:>3} | {}", i + 1, lines[2]);
            println!("{}^", (0..tok - (pos - 6)).map(|_| "~").collect::<String>());
            return;
        }
        pos += lines[2].len() + 1;
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



pub trait Walker {
    fn entity(&mut self, _: &ast::Entity) { }
    fn decl(&mut self, _: &ast::Decl) { }
    fn seq(&mut self, _: &ast::Seq) { }
}

pub trait Walkable {
    fn walk<W: Walker>(&self, walker: &mut W);
}

impl Walkable for ast::Entity {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.entity(self);
        for decl in &self.2 {
            decl.walk(walker);
        }
    }
}

impl Walkable for ast::Decl {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.decl(self);
        match *self {
            ast::Decl::On(_, ref block) => {
                block.walk(walker);
            }
            _ => { }
        }
    }
}

impl Walkable for ast::SeqBlock {
    fn walk<W: Walker>(&self, walker: &mut W) {
        for seq in self.0.iter() {
            seq.walk(walker);
        }
    }
}

impl Walkable for ast::Seq {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.seq(self);
        match *self {
            ast::Seq::If(ref cond, ref t, ref f) => {
                // TODO cond
                t.walk(walker);
                if let &Some(ref block) = f {
                    block.walk(walker);
                }
            }
            ast::Seq::Loop(ref body) => {
                body.walk(walker);
            }
            ast::Seq::While(ref cond, ref body) => {
                // TODO cond
                body.walk(walker);
            }
            _ => { }
        }
    }
}


pub struct InitWalker {
    init: HashMap<ast::Ident, ast::Expr>,
}

impl InitWalker {
    fn new() -> InitWalker {
        InitWalker {
            init: HashMap::new(),
        }
    }
}

impl Walker for InitWalker {
    fn decl(&mut self, item: &ast::Decl) {
        match *item {
            ast::Decl::Reg(ref ident, _, ref init) => {
                if let &Some(ref init) = init {
                    self.init.insert(ident.clone(), init.clone());
                }
            }
            _ => { }
        }
    }
}


pub struct CountWalker {
    yield_count: usize,
    fsm_transition_count: usize,
}

impl CountWalker {
    fn new() -> CountWalker {
        CountWalker {
            yield_count: 0,
            fsm_transition_count: 0,
        }
    }
}

impl Walker for CountWalker {
    fn seq(&mut self, item: &ast::Seq) {
        match *item {
            ast::Seq::Yield => {
                self.yield_count += 1;
            }
            ast::Seq::FsmTransition(..) => {
                self.fsm_transition_count += 1;
            }
            _ => { }
        }
    }
}


//pub struct PlaceholderReplacer;
//
//impl PlaceholderReplacer {
//    fn new() -> PlaceholderReplacer {
//        PlaceholderReplacer
//    }
//}
//
//impl Walker for PlaceholderReplacer {
//    fn expr(&mut self, item: &ast::Expr) {
//        match *item {
//            ast::Seq::Placeholder => {
//                *self = ast::Expr::VerilogLiteral("1'bx")
//            }
//            _ => { }
//        }
//    }
//}



//TODO not abort
pub fn typecheck(code: &ast::Code) {
    // TODO iterate through code, identify type decls. Then check AST for
    // incorrect references.
    //panic!("okay");
}
