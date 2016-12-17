#[macro_use] extern crate itertools;
#[macro_use] extern crate maplit;
#[macro_use] extern crate matches;
#[macro_use] extern crate lazy_static;
extern crate hoodlum_parser;
extern crate lalrpop_util;

pub mod async;

pub use hoodlum_parser::{ParseError, ast, hdl_parser};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use async::fsm_rewrite;
use std::sync::RwLock;

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



#[derive(Clone, Debug)]
pub struct VerilogState {
    indent: String,
    init: HashMap<ast::Ident, ast::Expr>,
    fsm: HashMap<u32, i32>,
}

impl VerilogState {
    pub fn new() -> VerilogState {
        VerilogState {
            indent: "".to_string(),
            init: HashMap::new(),
            fsm: HashMap::new(),
        }
    }

    pub fn tab(&self) -> VerilogState {
        VerilogState {
            indent: format!("{}    ", self.indent),
            init: self.init.clone(),
            fsm: self.fsm.clone(),
        }
    }

    pub fn untab(&self) -> VerilogState {
        VerilogState {
            indent: self.indent.chars().skip(4).collect(),
            init: self.init.clone(),
            fsm: self.fsm.clone(),
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
            ast::Op::Lte => "<=",
            ast::Op::Gte => ">=",
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
            ast::UnaryOp::Not => "~",
        }).to_string()
    }
}

impl ToVerilog for ast::Decl {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Decl::Latch(ref i, ref e) => {
                let mut dims = vec![];
                for item in e {
                    dims.push(format!("[({})-1:0]", item.to_verilog(v)));
                }
                let dim0 = if dims.len() > 0 {
                    Some(dims.remove(0))
                } else {
                    None
                };

                format!("{ind}reg{dim0} {name}{dims}{value};\n",
                    ind=v.indent,
                    dim0=if dim0.is_some() { format!(" {}", dim0.unwrap()) } else { " [(1)-1:0]".to_string() },
                    name=i.to_verilog(v),
                    dims=if dims.len() > 0 { format!(" {}", dims.join(" ")) } else { "".to_string() },
                    value=if dims.len() > 0 { format!("") } else { format!(" = 0") })
            }
            ast::Decl::Reg(ref i, ref e, ref value) => {
                let mut dims = vec![];
                for item in e {
                    dims.push(format!("[({})-1:0]", item.to_verilog(v)));
                }
                let dim0 = if dims.len() > 0 {
                    Some(dims.remove(0))
                } else {
                    None
                };

                let name = i.to_verilog(v);
                match (dims.len() > 0, value) {
                    // Hack for multidimensional array assignment
                    (true, &Some(ast::Expr::Concat(ref values))) => {
                        format!("{ind}reg{dim0} {name}{dims};\n{ind}always @(*) begin\n{value}{ind}end\n",
                            ind=v.indent,
                            dim0=if dim0.is_some() { format!(" {}", dim0.unwrap()) } else { " [(1)-1:0]".to_string() },
                            name=name,
                            dims=if dims.len() > 0 { format!(" {}", dims.join(" ")) } else { "".to_string() },
                            value=values.iter().enumerate().map(|(idx, x)| {
                                format!("{ind}{name}[{idx}] = {value};\n",
                                    ind=v.tab().indent,
                                    name=i.to_verilog(v),
                                    idx=idx,
                                    value=x.to_verilog(v))
                            }).collect::<Vec<_>>().join(""))
                    },
                    _ => {
                        format!("{ind}reg{dim0} {name}{dims};\n{value}",
                            ind=v.indent,
                            dim0=if dim0.is_some() { format!(" {}", dim0.unwrap()) } else { " [(1)-1:0]".to_string() },
                            name=name,
                            dims=if dims.len() > 0 { format!(" {}", dims.join(" ")) } else { "".to_string() },
                            value=if let &Some(ref value) = value {
                                format!("{ind}always @(*) {name} = {value};\n",
                                    ind=v.indent,
                                    name=i.to_verilog(v),
                                    value=value.to_verilog(v))
                            } else {
                                "".to_string()
                            })
                    }
                }
            }
            ast::Decl::Let(ref i, ref entity, ref args) => {
                format!("{ind}{entity} {i}({args});\n",
                    ind=v.indent,
                    entity=entity.to_verilog(v),
                    i=i.to_verilog(v),
                    args=args.iter().map(|x| {
                        if matches!(x.1, ast::Expr::Placeholder) {
                            format!(".{} ()", x.0.to_verilog(v))
                        } else {
                            format!(".{} ({})", x.0.to_verilog(v), x.1.to_verilog(v))
                        }
                    }).collect::<Vec<_>>().join(&format!(",\n{}", v.tab().indent)))
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
            ast::Decl::Always(ref block) => {
                format!("{ind}always @(*) begin\n{body}{ind}end\n",
                    ind=v.indent,
                    body=block.to_verilog(&v.tab()))
            }
        }
    }
}

impl ToVerilog for ast::SeqBlock {
    fn to_verilog(&self, v: &VerilogState) -> String {
        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
    }
}

// TODO get rid of this with rewriting AST
lazy_static! {
    static ref FSM_MAP: RwLock<HashMap<String, i32>> = RwLock::new(HashMap::new());
    static ref IS_MATCH: RwLock<bool> = RwLock::new(false);
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
                        if e.0.len() == 1 && matches!(e.0[0], ast::Seq::If(..)) {
                            let if_body = e.0[0].to_verilog(v);
                            format!("{ind}else {body}",
                                ind=v.indent,
                                body=if_body.trim_left())
                        } else {
                            format!("{ind}else begin\n{body}{ind}end\n",
                                ind=v.indent,
                                body=e.to_verilog(&v.tab()))
                        }
                    }))
            },
            ast::Seq::Set(ref block_type, ref id, ref value) => {
                format!("{ind}{name} {block} {value};\n",
                    ind=v.indent,
                    block=block_type.to_verilog(v),
                    name=id.to_verilog(v),
                    value=value.to_verilog(v))
            }
            ast::Seq::SetIndex(ref block_type, ref id, ref index, ref value) => {
                format!("{ind}{name}[{index}] {block} {value};\n",
                    ind=v.indent,
                    name=id.to_verilog(v),
                    index=index.to_verilog(v),
                    block=block_type.to_verilog(v),
                    value=value.to_verilog(v))
            }
            ast::Seq::SetRange(ref block_type, ref id, ref from, ref to, ref value) => {
                format!("{ind}{name}[{to}-1:{from}] {block} {value};\n",
                    ind=v.indent,
                    name=id.to_verilog(v),
                    from=from.to_verilog(v),
                    to=to.to_verilog(v),
                    block=block_type.to_verilog(v),
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
                                if matches!(*x, ast::Expr::Placeholder) {
                                    format!("default")
                                } else {
                                    *IS_MATCH.write().unwrap() = true;
                                    let ret = x.to_verilog(v);
                                    *IS_MATCH.write().unwrap() = false;
                                    ret
                                }
                            }).collect::<Vec<_>>().join(", "),
                            body=arm.1.to_verilog(&v.tab().tab()))
                    }).collect::<Vec<_>>().join(""))
            }
            ast::Seq::FsmCase(ref arms) => {
                format!("{ind}case (_FSM)\n{body}{ind}endcase\n",
                    ind=v.indent,
                    body=arms.iter().map(|arm| {
                        format!("{ind}{cond}: begin\n{body}{ind}end\n",
                            ind=v.tab().indent,
                            cond=if arm.0.is_empty() {
                                panic!("need match in fsm");
                            } else {
                                arm.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
                            },
                            body=arm.1.to_verilog(&v.tab().tab()))
                    }).collect::<Vec<_>>().join(""))
            }
            ast::Seq::Async(..) => {
                let (res, v_new) = fsm_rewrite(self, v);
                res.to_verilog(&v_new)
            }
            ast::Seq::FsmTransition(n) => {
                format!("{ind}_FSM <= {id};\n",
                    ind=v.indent,
                    id=n)
                    //id=v.fsm.get(&n).map(|x| x.to_string()).unwrap_or(format!("$$${}$$$", n))) //.expect(format!("Missing FSM state in generation step: {:?}!"))
                    //id=v.fsm.get(&n).expect(&format!("Missing FSM state in generation step: {:?}", n)))
            }
            ast::Seq::Fsm(ref arms) => {
                let mut states: HashMap<String, i32> = hashmap!{};
                let mut len = 0;
                for arm in arms {
                    states.insert((arm.0).0.clone(), len);
                    len += 1;
                }

                let mut out: Vec<(Vec<i32>, ast::SeqBlock)> = vec![];
                for arm in arms {
                    out.push((
                        vec![*states.get(&(arm.0).0).unwrap()],
                        arm.1.clone()
                    ));
                }

                *FSM_MAP.write().unwrap() = states;

                ast::Seq::FsmCase(out).to_verilog(&v)
            }
            ast::Seq::FsmCaseTransition(ref ident) => {
                format!("{ind}_FSM = {id};\n",
                    ind=v.indent,
                    id=FSM_MAP.read().unwrap().get(&ident.0).expect("Unknown fsm transition"))
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

//impl ToVerilog for ast::CombBlock {
//    fn to_verilog(&self, v: &VerilogState) -> String {
//        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
//    }
//}


impl ToVerilog for ast::BlockType {
    fn to_verilog(&self, _: &VerilogState) -> String {
        match self {
            &ast::BlockType::Blocking => "=".to_string(),
            &ast::BlockType::NonBlocking => "<=".to_string(),
        }
    }
}

//impl ToVerilog for ast::Comb {
//    fn to_verilog(&self, v: &VerilogState) -> String {
//        match *self {
//            ast::Comb::If(ref c, ref t, ref f) => {
//                format!("{ind}if ({cond}) begin\n{body}{ind}end\n{f}",
//                    ind=v.indent,
//                    cond=c.to_verilog(v),
//                    body=t.to_verilog(&v.tab()),
//                    f=f.as_ref().map_or("".to_string(), |e| {
//                        format!("{ind}else begin\n{body}{ind}end\n",
//                            ind=v.indent,
//                            body=e.to_verilog(&v.tab()))
//                    }))
//            },
//            ast::Comb::Assign(ref id, ref value) => {
//                format!("{ind}{name} = {value};\n",
//                    ind=v.indent,
//                    name=id.to_verilog(v),
//                    value=value.to_verilog(v))
//            }
//        }
//    }
//}

impl ToVerilog for ast::Expr {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Expr::Num(v) => format!("{}", v),
            ast::Expr::Ref(ref id) => id.to_verilog(v),
            ast::Expr::Slice(ref id, ref from, ref to) => {
                format!("{}[{}]",
                    id.to_verilog(v),
                    match *to {
                        Some(ref to) => format!("({})-1:{}", from.to_verilog(v), to.to_verilog(v)),
                        None => from.to_verilog(v),
                    })
            }
            ast::Expr::Ternary(ref c, ref t, ref e) => {
                format!("({cond} ? {th} : {el})",
                    cond=c.to_verilog(v),
                    th=t.to_verilog(&v.tab()),
                    el=e.to_verilog(&v.tab()))
            },
            ast::Expr::Concat(ref body) => {
                format!("{{{}}}", body.iter().map(|x| {
                    x.to_verilog(v)
                }).collect::<Vec<_>>().join(", "))
            }
            ast::Expr::Repeat(ref body, ref count) => {
                format!("{{({}){{{}}}}}",
                    count.to_verilog(v),
                    body.to_verilog(v))
            }
            ast::Expr::Arith(ref op, ref l, ref r) => {
                format!("({left} {op} {right})",
                    left=l.to_verilog(v),
                    op=op.to_verilog(v),
                    right=r.to_verilog(v))
            }
            ast::Expr::Unary(ref op, ref r) => {
                format!("{op}({right})",
                    op=op.to_verilog(v),
                    right=r.to_verilog(v))
            }
            ast::Expr::FsmEq(ref set) => {
                format!("({})", set.iter()
                    .map(|x| format!("_FSM == {}", x))
                    .collect::<Vec<_>>()
                    .join(" || "))
            }
            ast::Expr::FsmNe(ref set) => {
                format!("({})", set.iter()
                    .map(|x| format!("_FSM != {}", x))
                    .collect::<Vec<_>>()
                    .join(" && "))
            }
            ast::Expr::Placeholder => {
                if *IS_MATCH.read().unwrap() {
                    format!("'b?")
                } else {
                    panic!("Placeholer cannot be compiled to Verilog.");
                }
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
                    format!("\n    {} [({})-1:0] {}", x.1.to_verilog(&v), len, x.0.to_verilog(&v))
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
