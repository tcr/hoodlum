extern crate lalrpop_util;
extern crate hoodlum_parser;

pub use hoodlum_parser::{ParseError, ast, hdl_parser};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::mem;

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
            ast::Decl::Reg(ref ident, ref init) => {
                if let &Some(ref init) = init {
                    self.init.insert(ident.clone(), init.clone());
                }
            }
            ast::Decl::RegArray(ref ident, _, ref init) => {
                if let &Some(ref init) = init {
                    self.init.insert(ident.clone(), init.clone());
                }
            }
            _ => { }
        }
    }
}

pub struct ResetWalker {
    modified: HashSet<ast::Ident>,
}

impl ResetWalker {
    fn new() -> ResetWalker {
        ResetWalker {
            modified: HashSet::new(),
        }
    }
}

impl Walker for ResetWalker {
    fn seq(&mut self, item: &ast::Seq) {
        match *item {
            ast::Seq::Set(ref ident, _) => {
                self.modified.insert(ident.clone());
            }
            _ => { }
        }
    }
}



#[derive(Clone, Debug)]
pub struct VerilogState {
    indent: String,
    init: HashMap<ast::Ident, ast::Expr>,
}

impl VerilogState {
    pub fn new() -> VerilogState {
        VerilogState {
            indent: "".to_string(),
            init: HashMap::new(),
        }
    }

    pub fn tab(&self) -> VerilogState {
        VerilogState {
            indent: format!("{}    ", self.indent),
            init: self.init.clone(),
        }
    }

    pub fn untab(&self) -> VerilogState {
        VerilogState {
            indent: self.indent.chars().skip(4).collect(),
            init: self.init.clone(),
        }
    }
}

impl Default for VerilogState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToVerilog {
    fn to_verilog(&self, v: &VerilogState) -> String;
}

impl ToVerilog for ast::Ident {
    fn to_verilog(&self, _: &VerilogState) -> String {
        self.0.clone()
    }
}

impl ToVerilog for ast::Dir {
    fn to_verilog(&self, _: &VerilogState) -> String {
        (match *self {
            ast::Dir::In => "input",
            ast::Dir::Out => "output",
        }).to_string()
    }
}

impl ToVerilog for ast::Edge {
    fn to_verilog(&self, _: &VerilogState) -> String {
        (match *self {
            ast::Edge::Pos => "posedge",
            ast::Edge::Neg => "negedge",
        }).to_string()
    }
}

impl ToVerilog for ast::EdgeRef {
    fn to_verilog(&self, v: &VerilogState) -> String {
        format!("{} {}", self.1.to_verilog(v), self.0.to_verilog(v))
    }
}

impl ToVerilog for ast::Op {
    fn to_verilog(&self, _: &VerilogState) -> String {
        (match *self {
            ast::Op::Add => "+",
            ast::Op::Sub => "-",
            ast::Op::Mul => "*",
            ast::Op::Div => "/",
            ast::Op::Eq => "==",
            ast::Op::And => "&&",
            ast::Op::Or => "||",
            ast::Op::Lt => "<",
            ast::Op::Gt => ">",
            ast::Op::Ne => "!=",
            ast::Op::BinAnd => "&",
            ast::Op::BinOr => "|",
            ast::Op::LShift => "<<",
            ast::Op::RShift => ">>",
        }).to_string()
    }
}

impl ToVerilog for ast::UnaryOp {
    fn to_verilog(&self, _: &VerilogState) -> String {
        (match *self {
            ast::UnaryOp::Not => "!",
        }).to_string()
    }
}

impl ToVerilog for ast::Decl {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Decl::Reg(ref i, ref init) => {
                format!("{ind}reg {name}{init};\n", ind=v.indent, name=i.to_verilog(v),
                    init=if init.is_some() {
                        " = 0"
                    } else {
                        ""
                    })
            }
            ast::Decl::RegArray(ref i, ref e, ref value) => {
                format!("{ind}reg [({len})-1:0] {name}{value};\n",
                    ind=v.indent,
                    len=e.to_verilog(v),
                    name=i.to_verilog(v),
                    value=if value.is_some() { " = 0" } else { "" })
            }
            ast::Decl::Let(ref i, ref entity, ref args) => {
                format!("{ind}{entity} {i}({args});\n",
                    ind=v.indent,
                    entity=entity.to_verilog(v),
                    i=i.to_verilog(v),
                    args=args.iter().map(|x| {
                        format!(".{} ({})", x.0.to_verilog(v), x.1.to_verilog(v))
                    }).collect::<Vec<_>>().join(", "))
            }
            ast::Decl::Const(ref name, ref value) => {
                format!("{ind}localparam {name} = {value};\n",
                    ind=v.indent,
                    name=name.to_verilog(v),
                    value=value.to_verilog(v))
            }
            ast::Decl::On(ref edge, ref block) => {
                format!("{ind}always @({edge}) begin\n{body}{ind}end\n",
                    edge=edge.to_verilog(v),
                    ind=v.indent,
                    body=block.to_verilog(&v.tab()))
            }
            ast::Decl::Always(ref block) => block.to_verilog(v),
        }
    }
}

impl ToVerilog for ast::SeqBlock {
    fn to_verilog(&self, v: &VerilogState) -> String {
        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
    }
}

impl ToVerilog for ast::Seq {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Seq::If(ref c, ref t, ref f) => {
                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{f}",
                    ind=v.indent,
                    cond=c.to_verilog(v),
                    body=t.to_verilog(&v.tab()),
                    f=f.as_ref().map_or("".to_string(), |e| {
                        format!("{ind}else begin\n{body}{ind}end\n",
                            ind=v.indent,
                            body=e.to_verilog(&v.tab()))
                    }))
            },
            ast::Seq::Reset(ref c, ref b) => {
                let mut reset = ResetWalker::new();
                b.walk(&mut reset);

                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{ind}else begin\n{reset}{ind}end\n",
                    ind=v.indent,
                    cond=c.to_verilog(v),
                    body=b.to_verilog(&v.tab()),
                    reset=v.init.iter()
                        .filter(|&(ident, _)| reset.modified.contains(ident))
                        .map(|(ident, init)| {
                            ast::Seq::Set(ident.clone(), init.clone()).to_verilog(&v.tab())
                        }).collect::<Vec<_>>().join(""))
            },
            ast::Seq::Set(ref id, ref value) => {
                format!("{ind}{name} <= {value};\n",
                    ind=v.indent,
                    name=id.to_verilog(v),
                    value=value.to_verilog(v))
            }
            ast::Seq::SetIndex(ref id, ref index, ref value) => {
                format!("{ind}{name}[{index}] <= {value};\n",
                    ind=v.indent,
                    name=id.to_verilog(v),
                    index=index.to_verilog(v),
                    value=value.to_verilog(v))
            }
            ast::Seq::Match(ref cond, ref arms) => {
                format!("{ind}case ({cond})\n{body}{ind}endcase\n",
                    ind=v.indent,
                    cond=cond.to_verilog(v),
                    body=arms.iter().map(|arm| {
                        format!("{ind}{cond}: begin\n{body}{ind}end\n",
                            ind=v.tab().indent,
                            cond=arm.0.iter().map(|x| {
                                x.to_verilog(v)
                            }).collect::<Vec<_>>().join(", "),
                            body=arm.1.to_verilog(&v.tab().tab()))
                    }).collect::<Vec<_>>().join(""))
            }
            ast::Seq::Fsm(..) => {
                fsm_rewrite(self).to_verilog(v)
            }
            ast::Seq::Await(..) => {
                unreachable!("Cannot not compile Await statement to Verilog.")
            }
            ast::Seq::While(..) => {
                unreachable!("Cannot not compile While statement to Verilog.")
            }
            ast::Seq::Loop(..) => {
                unreachable!("Cannot not compile Loop statement to Verilog.")
            }
            ast::Seq::Yield => {
                unreachable!("Cannot not compile Yield statement to Verilog.")
            }
        }
    }
}

impl ToVerilog for ast::CombBlock {
    fn to_verilog(&self, v: &VerilogState) -> String {
        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
    }
}

impl ToVerilog for ast::Comb {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Comb::If(ref c, ref t, ref f) => {
                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{f}",
                    ind=v.indent,
                    cond=c.to_verilog(v),
                    body=t.to_verilog(&v.tab()),
                    f=f.as_ref().map_or("".to_string(), |e| {
                        format!("{ind}else begin\n{body}{ind}end\n",
                            ind=v.indent,
                            body=e.to_verilog(&v.tab()))
                    }))
            },
            ast::Comb::Assign(ref id, ref value) => {
                format!("{ind}assign {name} = {value};\n",
                    ind=v.indent,
                    name=id.to_verilog(v),
                    value=value.to_verilog(v))
            }
        }
    }
}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Expr::Num(v) => format!("{}", v),
            ast::Expr::Ref(ref id) => id.to_verilog(v),
            ast::Expr::Slice(ref id, ref from, ref to) => {
                format!("{}[{}{}]", id.to_verilog(v), from.to_verilog(v),
                    to.as_ref().map_or("".to_string(), |x| {
                        format!(":{}", x.to_verilog(v))
                    }))
            }
            ast::Expr::Concat(ref body) => {
                format!("{{{}}}", body.iter().map(|x| {
                    x.to_verilog(v)
                }).collect::<Vec<_>>().join(", "))
            }
            ast::Expr::Arith(ref op, ref l, ref r) => {
                format!("{left} {op} {right}",
                    left=l.to_verilog(v),
                    op=op.to_verilog(v),
                    right=r.to_verilog(v))
            }
            ast::Expr::Unary(ref op, ref r) => {
                format!("{op}({right})",
                    op=op.to_verilog(v),
                    right=r.to_verilog(v))
            }
        }
    }
}

impl ToVerilog for ast::Entity {
    fn to_verilog(&self, v: &VerilogState) -> String {
        let mut walker = InitWalker::new();
        self.walk(&mut walker);

        let mut v = v.clone();
        v.init = walker.init;

        format!("{ind}module {name} ({args}\n);\n{body}{ind}endmodule\n\n",
            ind=v.indent,
            name=self.0.to_verilog(&v),
            args=self.1.iter().map(|x| {
                if let Some(len) = x.2 {
                    format!("\n    {} [{}:0] {}", x.1.to_verilog(&v), len, x.0.to_verilog(&v))
                } else {
                    format!("\n    {} {}", x.1.to_verilog(&v), x.0.to_verilog(&v))
                }
            }).collect::<Vec<_>>().join(","),
            body=self.2.iter().map(|x| {
                x.to_verilog(&v.tab())
            }).collect::<Vec<_>>().join(""))
    }
}

impl ToVerilog for ast::Code {
    fn to_verilog(&self, v: &VerilogState) -> String {
        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
    }
}

struct FsmState {
    counter: u32,
}

impl FsmState {
    fn next(&mut self) -> u32 {
        let ret = self.counter;
        self.counter += 1;
        ret
    }
}

fn fsm_list(fsm: &mut FsmState, input: &[ast::Seq]) -> Vec<(Vec<u32>, Vec<ast::Seq>)> {
    let mut ret = vec![];
    let mut cur: (Vec<u32>, Vec<ast::Seq>) = (vec![fsm.next()], vec![]);
    for item in input.iter().rev() {
        let bundle = match item.clone() {
            ast::Seq::Loop(body) => {
                vec![ast::Seq::While(ast::Expr::Num(1), body)]
            }
            ast::Seq::Await(cond) => {
                vec![ast::Seq::Yield, ast::Seq::While(ast::Expr::Unary(ast::UnaryOp::Not, Box::new(cond)), ast::SeqBlock(vec![ast::Seq::Yield]))]
            }
            seq => {
                vec![seq]
            }
        };

        for item in bundle.iter().rev() {
            match item {
                &ast::Seq::While(ref cond, ast::SeqBlock(ref inner)) => {
                    let mut list = fsm_list(fsm, &inner);

                    // Make mutable clone of cond.
                    let mut cond = cond.clone();

                    // Remove current segment as else statement.
                    let else_seq = if !cur.1.is_empty() {
                        let mut new_cur = (vec![], vec![]);
                        new_cur.0.extend(&cur.0);

                        // If the last statement isn't a redundant FSM statement, need a transition
                        if let Some(&ast::Seq::Set(..)) = cur.1.last() {
                            cur.0.remove(0);
                        } else {
                            let id = cur.0[0];
                            cur.1.insert(0, ast::Seq::Set(ast::Ident("_FSM".to_string()), ast::Expr::Num(id as i32)));
                        }
                        // Add all remaining states.
                        for item in &cur.0 {
                            cond = ast::Expr::Arith(ast::Op::And,
                                Box::new(ast::Expr::Arith(ast::Op::Ne,
                                    Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
                                    Box::new(ast::Expr::Num(*item as i32)))),
                                Box::new(cond));
                        }

                        Some(ast::SeqBlock(mem::replace(&mut cur, new_cur).1))
                    } else {
                        None
                    };

                    // Extract last statement
                    if list.len() < 2 {
                        panic!("Can't have empty while body");
                    }
                    let last = list.pop().unwrap();
                    let mut first = list.remove(0);
                    ret.extend(list);

                    // Extend matching states. (We adopt the first state)
                    cur.0.extend(&first.0);
                    if !last.1.is_empty() {
                        cur.0.extend(&last.0);
                        // TODO can we recover the trashed state number?
                    }

                    // Remove the last state transition.
                    if last.1.is_empty() {
                        first.1.pop();
                    }
                    // Construct if statement.
                    cur.1.insert(0, ast::Seq::If(cond,
                        ast::SeqBlock(first.1),
                        else_seq));

                    // Check last block.
                    if !last.1.is_empty() {
                        cur.1.insert(0, ast::Seq::If(ast::Expr::Num(1),
                            ast::SeqBlock(last.1),
                            None));
                    }
                }
                &ast::Seq::Yield => {
                    let prev = fsm.counter - 1;

                    let old_cur = mem::replace(&mut cur, (vec![fsm.counter], vec![]));
                    fsm.counter += 1;
                    ret.insert(0, old_cur);

                    cur.1.insert(0, ast::Seq::Set(ast::Ident("_FSM".to_string()), ast::Expr::Num(prev as i32)));
                }
                &ast::Seq::Fsm(..) => {
                    panic!("Cannot deconstruct nested FSMs");
                }
                seq => {
                    cur.1.insert(0, seq.clone());
                }
            }
        }
    }

    ret.insert(0, cur);
    ret
}

fn fsm_rewrite(input: &ast::Seq) -> ast::Seq {
    let mut fsm = FsmState {
        counter: 0,
    };

    if let &ast::Seq::Fsm(ref inner) = input {
        let mut res = inner.0.clone();
        res.push(ast::Seq::Yield);

        let mut ret = fsm_list(&mut fsm, &res);
        // Remove last yield block.
        ret.pop();
        ret[0].0.insert(0, 0);

        return ast::Seq::Match(ast::Expr::Ref(ast::Ident("_FSM".to_string())),
            ret.iter().map(|x| {
                let mut states = x.0.clone();
                states.sort();
                (states.iter().rev().map(|x| ast::Expr::Num(*x as _)).collect(), ast::SeqBlock(x.1.clone()))
            }).collect())
    } else {
        panic!("not fsm");
    }
}


#[test]
fn rewrite_fsm() {
    let code = r#"
fsm {
    while !tx_trigger {
        spi_tx <= 0;
        yield;
    }

    read_index <= 7;
    spi_tx <= tx_byte[7];
    tx_ready <= 0;
    yield;

    while read_index > 0 {
        spi_tx <= tx_byte[read_index - 1];
        read_index <= read_index - 1;
        yield;
    }

    tx_ready <= 1;

    loop {
        a <= 1;
        yield;
        a <= 2;
    }
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0: begin
        if (!tx_trigger) begin
            spi_tx <= 0;
        end
        else begin
            read_index <= 7;
            spi_tx <= tx_byte[7];
            tx_ready <= 0;
            FSM <= 1;
        end
    end
    1, 2: begin
        if (FSM != 2 && read_index > 0) begin
            spi_tx <= tx_byte[read_index - 1];
            read_index <= read_index - 1;
        end
        else begin
            if (FSM == 1) begin
                tx_ready <= 1;
            end
            if (FSM == 2) begin
                a <= 2;
            end
            a <= 1;
            FSM <= 2;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_fsm2() {
    let code = r#"
fsm {
    while !tx_trigger {
        spi_tx <= 0;
        yield;
    }

    read_index <= 7;
    spi_tx <= tx_byte[7];
    tx_ready <= 0;
    yield;

    while read_index > 0 {
        spi_tx <= tx_byte[read_index - 1];
        read_index <= read_index - 1;
        yield;
    }

    tx_ready <= 1;

    loop {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0: begin
        if (!tx_trigger) begin
            spi_tx <= 0;
        end
        else begin
            read_index <= 7;
            spi_tx <= tx_byte[7];
            tx_ready <= 0;
            FSM <= 1;
        end
    end
    1, 2: begin
        if (FSM != 2 && read_index > 0) begin
            spi_tx <= tx_byte[read_index - 1];
            read_index <= read_index - 1;
        end
        else begin
            if (FSM == 1) begin
                tx_ready <= 1;
            end
            a <= 1;
            FSM <= 2;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        a <= 1;
        yield;
        b <= 1;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1: begin
        if (FSM == 0) begin
            spi_tx <= 0;
        end
        if (FSM == 1) begin
            b <= 1;
        end
        if (!tx_trigger) begin
            a <= 1;
            FSM <= 1;
        end
        else begin
            spi_tx <= 1;
            FSM <= 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await2() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        a <= 1;
        yield;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1: begin
        if (FSM == 0) begin
            spi_tx <= 0;
        end
        if (!tx_trigger) begin
            a <= 1;
            FSM <= 1;
        end
        else begin
            spi_tx <= 1;
            FSM <= 0;
        end
    end
endcase
"#);
}

#[test]
fn rewrite_await3() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        yield;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1: begin
        if (FSM == 0) begin
            spi_tx <= 0;
        end
        if (!tx_trigger) begin
            FSM <= 1;
        end
        else begin
            spi_tx <= 1;
            FSM <= 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await4() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        yield;
    }

    spi_tx <= 1;

    loop { yield; }
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1, 2: begin
        if (FSM == 0) begin
            spi_tx <= 0;
        end
        if (FSM != 2 && !tx_trigger) begin
            FSM <= 1;
        end
        else begin
            if (FSM == 0 || FSM == 1) begin
                spi_tx <= 1;
            end
            FSM <= 2;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await5() {
    let code = r#"
fsm {
    LED1 <= 1;

    CS <= 0;
    tx_valid <= 1;
    tx_byte <= 0x22;
    await !spi_ready;
    await spi_ready;
    yield;
    tx_byte <= 0x16;
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1, 2: begin
        if (FSM == 0) begin
            LED1 <= 1;
            CS <= 0;
            tx_valid <= 1;
            tx_byte <= 34;
        end
        if (FSM != 2 && FSM != 3 && !!spi_ready) begin
            FSM <= 1;
        end
        else begin
            if (FSM != 3 && !spi_ready) begin
                FSM <= 2;
            end
            else begin
                FSM <= 3;
            end
        end
    end
    3: begin
        tx_byte <= 22;
        FSM <= 0;
    end
endcase
"#);
}


#[test]
fn rewrite_await6() {
    let code = r#"
fsm {
    LED1 <= 1;

    CS <= 0;
    tx_valid <= 1;
    tx_byte <= 0x22;
    while !spi_ready { yield; }
    while spi_ready { yield; }
    tx_byte <= 0x16;
}
"#;

    let res = parse_results(code, self::hdl_parser::parse_SeqStatement(code));

    let res = fsm_rewrite(&res);
    let out = res.to_verilog(&Default::default());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0, 1, 2: begin
        if (FSM == 0) begin
            LED1 <= 1;
            CS <= 0;
            tx_valid <= 1;
            tx_byte <= 34;
        end
        if (FSM != 2 && FSM != 3 && !!spi_ready) begin
            FSM <= 1;
        end
        else begin
            if (FSM != 3 && !spi_ready) begin
                FSM <= 2;
            end
            else begin
                FSM <= 3;
            end
        end
    end
    3: begin
        tx_byte <= 22;
        FSM <= 0;
    end
endcase
"#);
}
