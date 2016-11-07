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
                let (res, v_new) = fsm_rewrite(self, v);
                res.to_verilog(&v_new)
            }
            ast::Seq::FsmTransition(n) => {
                format!("{ind}_FSM <= {id};\n",
                    ind=v.indent,
                    id=v.fsm.get(&n).map(|x| x.to_string()).unwrap_or(format!("$$${}$$$", n))) //.expect(format!("Missing FSM state in generation step: {:?}!"))
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
            ast::Expr::FsmValue(ref state) => {
                format!("{state}",
                    state=v.fsm.get(state).expect("Missing FsmValue value!"))
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

struct FsmCounter {
    counter: u32,
}

impl FsmCounter {
    fn next(&mut self) -> u32 {
        let ret = self.counter;
        self.counter += 1;
        ret
    }
}

fn invert_expr(expr: ast::Expr) -> ast::Expr {
    if let ast::Expr::Unary(ast::UnaryOp::Not, inner) = expr {
        *inner
    } else {
        ast::Expr::Unary(ast::UnaryOp::Not, Box::new(expr))
    }
}

#[derive(Debug, Clone)]
struct FsmState {
    id: u32,
    contains: Vec<u32>,
    body: Vec<ast::Seq>,
}

impl FsmState {
    fn all_states(&self) -> Vec<u32> {
        let mut ret = vec![self.id];
        ret.extend(&self.contains);
        ret
    }
}

fn fsm_match_list(op: ast::Op, list: &[u32]) -> Option<ast::Expr> {
    if list.len() == 0 {
        return None;
    }

    let mut cond = ast::Expr::Arith(op.clone(),
        Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
        Box::new(ast::Expr::FsmValue(list[0])));
    for item in &list[1..] {
        cond = ast::Expr::Arith(ast::Op::And,
            Box::new(ast::Expr::Arith(op.clone(),
                Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
                Box::new(ast::Expr::FsmValue(*item)))),
            Box::new(cond));
    }
    Some(cond)
}

fn fsm_list(fsm: &mut FsmCounter, input: &[ast::Seq], has_prev: bool) -> Vec<FsmState> {
    let mut ret: Vec<FsmState> = vec![];
    let mut cur = FsmState {
        id: fsm.next(),
        contains: vec![],
        body: vec![]
    };

    // Expand easy input tokens.
    let mut new_input = vec![];
    for item in input {
        match item {
            &ast::Seq::Loop(ref body) => {
                new_input.push(ast::Seq::While(ast::Expr::Num(1), body.clone()));
            }
            &ast::Seq::Await(ref cond) => {
                new_input.push(ast::Seq::Yield);
                new_input.push(ast::Seq::While(invert_expr(cond.clone()),
                        ast::SeqBlock(vec![ast::Seq::Yield])));
            }
            ref seq => {
                new_input.push((*seq).clone());
            }
        };
    }

    // Iterate through input.
    while let Some(item) = new_input.pop() {
        match item {
            ast::Seq::While(ref cond, ast::SeqBlock(ref inner)) => {
                let mut list = fsm_list(fsm, &inner, false);

                // Remove our last and first states.
                if list.len() < 2 {
                    panic!("Can't have empty while body");
                }
                let last = list.pop().unwrap();
                let mut first = list.remove(0);

                // Make mutable clone of cond.
                let mut cond = cond.clone();

                // What output we've transformed so far, we use as the else { ... }
                // block in an if statement that functions as our loop.
                let else_seq = if !cur.body.is_empty() || has_prev {
                    let mut new_cur = FsmState {
                        id: cur.id,
                        contains: cur.contains.clone(),
                        body: vec![],
                    };

                    // If the last statement in our else block is a FSM transition,
                    // we don't need to insert a new one.
                    if let Some(&ast::Seq::FsmTransition(value)) = cur.body.last() {
                        new_cur.id = last.id;
                    } else {
                        cur.body.insert(0, ast::Seq::FsmTransition(cur.id));
                    }

                    // Create a condition excluding all else { ... } FSM states.
                    // Add this to our loop's if { ... } condition.
                    if let Some(matcher) = fsm_match_list(ast::Op::Ne, &cur.contains) {
                        cond = ast::Expr::Arith(ast::Op::And,
                            Box::new(matcher),
                            Box::new(cond));
                    }

                    // Extract our else { ... } block.
                    let old_cur = mem::replace(&mut cur, new_cur);
                    Some(ast::SeqBlock(old_cur.body))
                } else {
                    None
                };

                //println!("FIRST {:?}", first.id);
                //println!("LAST {:?}", last.id);

                // All other states will be preserved.
                ret.extend(list);

                // Extend matching states to include the first and last entry.
                cur.contains.extend(&first.contains);
                // If the last entry is empty, we have no state transition.
                if !(last.body.is_empty() && new_input.is_empty()) {
                    if cur.id != last.id {
                        cur.contains.extend(last.all_states());
                    } else {
                        cur.contains.extend(&last.contains);
                    }
                }

                // Remove the first state transition if the last state is empty.
                // We will need it, though, if we have preceding content.
                if last.body.is_empty() && new_input.is_empty() {
                    first.body.pop();
                }

                // If the first body is empty, we can just negate the condition.
                // Otherwise, we construct a proper if { ... } else { ... } block.
                if first.body.is_empty() && else_seq.is_some() {
                    // Construct if statement without else body.
                    cur.body.insert(0, ast::Seq::If(invert_expr(cond),
                        else_seq.unwrap(),
                        None));
                } else {
                    // Construct if statement with else body.
                    cur.body.insert(0, ast::Seq::If(cond,
                        ast::SeqBlock(first.body),
                        else_seq));
                }

                // The last block gets inserted before our if block, as it is
                // run first.
                if !last.body.is_empty() {
                    cur.body.insert(0, ast::Seq::If(fsm_match_list(ast::Op::Eq, &[last.id]).unwrap(),
                        ast::SeqBlock(last.body),
                        None));
                }

                // If our if statement has preceding content, it needs to be
                // run in sequence with our if block; but we don't want to run
                // it on every loop iteration. So we nest it in an if statement
                // exclusive to its FSM states.
                if !new_input.is_empty() {
                    let has_yield = if let Some(&ast::Seq::Yield) = new_input.last() {
                        true
                    } else {
                        false
                    };

                    if has_yield {
                        println!("ADJUSTING {:?}", cur.id);
                        fsm.counter = cur.id;
                        if cur.contains.len() > 0 {
                            fsm.counter += 1;
                        }
                    }

                    let mut rest = fsm_list(fsm, &new_input, true);

                    // If we actually have states to process, process them.
                    if !has_yield {
                        let rest_last = rest.pop().unwrap();

                        // Create a condition matching all FSM states.
                        let mut states = rest_last.all_states();
                        states.sort();
                        if states.len() > 1 {
                            states.remove(0);
                        }
                        let if_cond = fsm_match_list(ast::Op::Eq, &states).unwrap();

                        // Create a condition matching else FSM states.
                        let else_seq = mem::replace(&mut cur.body, vec![]);
                        let mut states = cur.all_states();
                        states.push(*rest_last.all_states().iter().min().unwrap());
                        let else_cond = fsm_match_list(ast::Op::Eq, &states).unwrap();

                        // Extend matching states.
                        cur.contains.extend(rest_last.all_states());

                        if rest_last.contains.len() > 0 {
                            cur.body.push(ast::Seq::If(else_cond,
                                ast::SeqBlock(else_seq),
                                None));
                        } else {
                            cur.body.extend(else_seq);
                        }
                        cur.body.insert(0, ast::Seq::If(if_cond,
                            ast::SeqBlock(rest_last.body),
                            None));
                    } else {
                        //let last_id = cur.id;
                        //cur.id = rest.last().unwrap().id;
                        println!("cur {:?}", rest);
                        if let &mut ast::Seq::If(ref mut cond, ast::SeqBlock(ref mut body), ref mut else_seq) = &mut cur.body[0] {
                            //if let &mut Some(ast::SeqBlock(ref mut else_seq)) = else_seq {
                            //    if let Some(&mut ast::Seq::FsmTransition(ref mut id)) = else_seq.get_mut(0) {
                            //        if *id == last_id {
                            //            *id = cur.id;
                            //        }
                            //    }
                            //}
                            body.pop();
                            if body.is_empty() && else_seq.is_some() {
                                *body = mem::replace(else_seq, None).unwrap().0;
                                *cond = invert_expr(cond.clone());
                            }
                        }
                            rest.pop();
                    }

                    ret.insert(0, cur);
                    for item in rest.into_iter().rev() {
                        ret.insert(0, item);
                    }
                    return ret;
                }
            }
            ast::Seq::Yield => {
                let old_cur = mem::replace(&mut cur, FsmState {
                    id: fsm.next(),
                    contains: vec![],
                    body: vec![],
                });

                cur.body.insert(0, ast::Seq::FsmTransition(old_cur.id));
                ret.insert(0, old_cur);
            }
            ast::Seq::Fsm(..) => {
                panic!("Cannot deconstruct nested FSMs");
            }
            seq => {
                cur.body.insert(0, seq.clone());
            }
        }
    }

    ret.insert(0, cur);
    ret
}

fn fsm_rewrite(input: &ast::Seq, v: &VerilogState) -> (ast::Seq, VerilogState) {
    let mut fsm = FsmCounter {
        counter: 0,
    };

    if let &ast::Seq::Fsm(ref inner) = input {
        let mut res = inner.0.clone();
        res.push(ast::Seq::Yield);

        let mut ret = fsm_list(&mut fsm, &res, false);
        // Remove last yield block.
        ret.pop();

        let mut map = HashMap::new();
        map.insert(0, 0);

        for item in &ret {
            let mut states = vec![item.id];
            states.extend(&item.contains);
            states.sort();
            for id in states.iter().rev() {
                if !map.get(id).is_some() {
                    if true {
                        let new_id = (map.values().len() - 1) as i32;
                        map.insert(*id, new_id);
                    } else {
                        map.insert(*id, *id as i32);
                    }
                }
            }
        }
        println!("STATES {:?}", map);

        let seq = ast::Seq::Match(ast::Expr::Ref(ast::Ident("_FSM".to_string())),
            ret.iter().map(|x| {
                let mut states: Vec<i32> = x.contains.iter().map(|x| *map.get(x).unwrap()).collect::<Vec<_>>();
                states.push(*map.get(&x.id).unwrap());
                states.sort();
                (states.iter().map(|x| ast::Expr::Num(*x as _)).collect(), ast::SeqBlock(x.body.clone()))
            }).collect());

        let mut v_new = v.clone();
        v_new.fsm = map;
        (seq, v_new)
    } else {
        panic!("not fsm");
    }
}
