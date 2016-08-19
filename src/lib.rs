use std::collections::{HashMap, HashSet};

pub mod hdl_parser;
pub mod ast;

#[macro_export]
macro_rules! hdl {
    ( $( $x:tt )* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(stringify!($x));
            )*
            let concat = temp_vec.join(" ");

            println!("Parsing {:?}", concat);
            let res = hdl_parser::parse_Code(&concat);

            res.unwrap()
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
                self.init.insert(ident.clone(), init.clone());
            }
            ast::Decl::RegArray(ref ident, _, ref init) => {
                self.init.insert(ident.clone(), init.clone());
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
        }).to_string()
    }
}

impl ToVerilog for ast::Decl {
    fn to_verilog(&self, v: &VerilogState) -> String {
        match *self {
            ast::Decl::Reg(ref i, _) => {
                format!("{ind}reg {name} = 0;", ind=v.indent, name=i.to_verilog(v))
            }
            ast::Decl::RegArray(ref i, ref e, _) => {
                format!("{ind}reg [({len})-1:0] {name} = 0;",
                    ind=v.indent,
                    len=e.to_verilog(v),
                    name=i.to_verilog(v))
            }
            ast::Decl::On(ref edge, ref block) => {
                format!("{ind}always @({edge}) begin\n{body}{ind}end",
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
            ast::Seq::Match(ref cond, ref arms) => {
                format!("{ind}case ({cond})\n{body}{ind}endcase\n",
                    ind=v.indent,
                    cond=cond.to_verilog(v),
                    body=arms.iter().map(|arm| {
                        format!("{ind}{cond}: begin\n{body}{ind}end\n",
                            ind=v.tab().indent,
                            cond=arm.0.to_verilog(&v.tab()),
                            body=arm.1.to_verilog(&v.tab().tab()))
                    }).collect::<Vec<_>>().join(""))
            }
            ast::Seq::Fsm(ref block) => {
                ast::Seq::Match(ast::Expr::Ref(ast::Ident("FSM".to_owned())), {
                    // Divide block.
                    let mut arms = vec![vec![]];
                    for seq in &block.0 {
                        if let &ast::Seq::Yield = seq {
                            arms.insert(0, vec![]);
                        } else {
                            arms[0].push(seq.clone());
                        }
                    }

                    let last = arms.len() - 1;

                    arms.into_iter().enumerate().map(|(i, mut seq)| {
                        let state = ast::Ident("FSM".to_string());
                        seq.push(ast::Seq::Set(state.clone(),
                            if i == last {
                                ast::Expr::Num(0)
                            } else {
                                ast::Expr::Arith(ast::Op::Add,
                                    Box::new(ast::Expr::Ref(state.clone())),
                                    Box::new(ast::Expr::Num(1)))
                            }));

                        (ast::Expr::Num(i as i32), ast::SeqBlock(seq))
                    }).collect()
                }).to_verilog(v)
            }
            ast::Seq::Yield => {
                format!("")
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
        }
    }
}

impl ToVerilog for ast::Entity {
    fn to_verilog(&self, v: &VerilogState) -> String {
        let mut walker = InitWalker::new();
        self.walk(&mut walker);

        let mut v = v.clone();
        v.init = walker.init;

        format!("{ind}module {name} ({args});\n{body}{ind}endmodule\n\n",
            ind=v.indent,
            name=self.0.to_verilog(&v),
            args=self.1.iter().map(|x| {
                format!("{} {}", x.1.to_verilog(&v), x.0.to_verilog(&v))
            }).collect::<Vec<_>>().join(", "),
            body=self.2.iter().map(|x| {
                x.to_verilog(&v.tab())
            }).collect::<Vec<_>>().join("\n"))
    }
}

impl ToVerilog for ast::Code {
    fn to_verilog(&self, v: &VerilogState) -> String {
        self.0.iter().map(|x| x.to_verilog(v)).collect::<Vec<_>>().join("")
    }
}
