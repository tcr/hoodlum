extern crate lalrpop_util;
extern crate hoodlum_parser;

pub use hoodlum_parser::{ParseError, ast, hdl_parser};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

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
                        "= 0"
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
                        x.to_verilog(v)
                    }).collect::<Vec<_>>().join(", "))
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
                format!("{op}{right}",
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
                if let Some(len) = x.2 {
                    format!("{} [{}:0] {}", x.1.to_verilog(&v), len, x.0.to_verilog(&v))
                } else {
                    format!("{} {}", x.1.to_verilog(&v), x.0.to_verilog(&v))
                }
            }).collect::<Vec<_>>().join(", "),
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

fn fsm_rewrite(input: &ast::Seq) -> ast::Seq {
    #[derive(Clone, Debug)]
    enum Fsm {
        Loop(Option<ast::Expr>, Vec<Vec<ast::Seq>>),
        Block(Vec<Vec<ast::Seq>>),
    }

    let mut input = if let &ast::Seq::Fsm(ast::SeqBlock(ref block)) = input {
        block.clone()
    } else {
        panic!("Called fsm_rewrite with non-fsm");
    };

    // Convert input to Fsm tree.
    let mut states = vec![];
    while !input.is_empty() {
        let item = input.remove(0);
        match item {
            ast::Seq::Await(cond) => {
                // while (!cond) { yield; }
                states.push(Fsm::Loop(Some(ast::Expr::Unary(ast::UnaryOp::Not, Box::new(cond))), vec![vec![], vec![]]));
            }
            ast::Seq::While(cond, ast::SeqBlock(inner)) => {
                states.push(Fsm::Loop(Some(cond), vec![vec![]]));

                for item in inner {
                    match item {
                        ast::Seq::While(..) => unreachable!(),
                        ast::Seq::Loop(..) => unreachable!(),
                        ast::Seq::Await(..) => unreachable!(),
                        ast::Seq::Yield => {
                            if let &mut Fsm::Loop(_, ref mut block) = states.last_mut().unwrap() {
                                block.push(vec![])
                            }
                        }
                        seq => {
                            if let &mut Fsm::Loop(_, ref mut block) = states.last_mut().unwrap() {
                                block.last_mut().unwrap().push(seq);
                            }
                        }
                    }
                }
            }
            ast::Seq::Loop(ast::SeqBlock(inner)) => {
                states.push(Fsm::Loop(None, vec![vec![]]));

                for item in inner {
                    match item {
                        ast::Seq::While(..) => unreachable!(),
                        ast::Seq::Loop(..) => unreachable!(),
                        ast::Seq::Await(..) => unreachable!(),
                        ast::Seq::Yield => {
                            if let &mut Fsm::Loop(_, ref mut block) = states.last_mut().unwrap() {
                                block.push(vec![])
                            }
                        }
                        seq => {
                            if let &mut Fsm::Loop(_, ref mut block) = states.last_mut().unwrap() {
                                block.last_mut().unwrap().push(seq);
                            }
                        }
                    }
                }
            }
            ast::Seq::Yield => {
                if let Some(&Fsm::Block(..)) = states.last() {
                } else {
                    states.push(Fsm::Block(vec![vec![]]));
                }

                if let &mut Fsm::Block(ref mut block) = states.last_mut().unwrap() {
                    block.push(vec![])
                }
            }
            seq => {
                if let Some(&Fsm::Block(..)) = states.last() {
                } else {
                    states.push(Fsm::Block(vec![vec![]]));
                }

                if let &mut Fsm::Block(ref mut block) = states.last_mut().unwrap() {
                    block.last_mut().unwrap().push(seq);
                }
            }
        }
    }

    println!("state {:?}", states);

    let mut output: Vec<(Vec<i32>, Vec<ast::Seq>)> = vec![];

    #[derive(Debug, Clone)]
    enum FsmState {
        Block(Vec<ast::Seq>),
        Loop(Option<ast::Expr>, Vec<ast::Seq>),
    }

    let mut state_start: i32 = 1;
    let mut state_match = vec![0];
    let mut state = vec![]; //FsmState::Block(vec![]);

    fn process(output: &mut Vec<(Vec<i32>, Vec<ast::Seq>)>, state_match: &Vec<i32>, state: Vec<FsmState>, last: bool) {
        // TODO

        let mut res = if !last {
            vec![
                ast::Seq::Set(ast::Ident("FSM".to_string()), ast::Expr::Num(*state_match.last().unwrap() + 1)),
            ]
        } else {
            vec![]
        };

        let last_match = *state_match.last().unwrap() + 1;
        let mut state_match = state_match.clone();
        //if last {
        //    state_match.push(last_match);
        //}

        println!("PROCESSING STEP {:?}", state);

        let mut loop_count = 0;
        for item in state.into_iter().rev() {
            if let FsmState::Block(mut body) = item {
                body.extend(res);
                res = body;
            } else if let FsmState::Loop(cond, mut body) = item {
                if let Some(mut cond) = cond {
                    if last && loop_count > 0 {
                        cond = ast::Expr::Arith(ast::Op::And,
                            Box::new(ast::Expr::Arith(ast::Op::Ne,
                                Box::new(ast::Expr::Ref(ast::Ident("FSM".to_string()))),
                                Box::new(ast::Expr::Num(*state_match.last().unwrap())))),
                            Box::new(cond));
                    }

                    res = vec![ast::Seq::If(cond, ast::SeqBlock(body), Some(ast::SeqBlock(res)))];
                } else {
                    body.extend(res);
                    res = body;
                }

                loop_count += 1;
            }
        }

        output.push((state_match, res));
    }

    while !states.is_empty() {
        let item = states.remove(0);

        println!("\nprocessing block...{:?}\n", item);
        match item {
            Fsm::Block(mut blocks) => {
                let mut block = blocks.remove(0);
                if states.is_empty() && blocks.is_empty() {
                    block.push(ast::Seq::Set(ast::Ident("FSM".to_string()), ast::Expr::Num(0)));
                }
                state.push(FsmState::Block(block));

                while !blocks.is_empty() {
                    process(&mut output, &state_match, state, false);

                    state = vec![];
                    state_match = vec![state_start];
                    state_start += 1;

                    let mut block = blocks.remove(0);
                    if states.is_empty() && blocks.is_empty() {
                        block.push(ast::Seq::Set(ast::Ident("FSM".to_string()), ast::Expr::Num(0)));
                    }
                    state.push(FsmState::Block(block));
                }
            }
            Fsm::Loop(expr, mut blocks) => {
                if blocks.len() < 2 {
                    panic!("While construct must contain yield statement.");
                }

                let mut initial = blocks.remove(0);

                // Get the final loop entry.
                let next = blocks.pop().unwrap();

                // Intermediate loop sequences are new steps.
                for item in blocks {
                    process(&mut output, &vec![state_start], vec![FsmState::Block(item)], false);
                    state_start += 1;
                }

                // Wrap previous block in an if statement.
                let prev_item: Option<FsmState> = state.last().map(|x| x.clone());
                let merged_prev_block = if let Some(FsmState::Block(content)) = prev_item {
                    // Only do this if there is any loop content at all.
                    //if !next.is_empty() || !initial.is_empty() {
                        state.pop();
                        if !content.is_empty() {
                            // Match all states up to here.
                            let mut conds = state_match.iter().map(|x| {
                                ast::Expr::Arith(ast::Op::Eq,
                                    Box::new(ast::Expr::Ref(ast::Ident("FSM".to_string()))),
                                    Box::new(ast::Expr::Num(*x)))
                            }).collect::<Vec<_>>();

                            let mut cond = conds.remove(0);
                            while !conds.is_empty() {
                                cond = ast::Expr::Arith(ast::Op::Or,
                                    Box::new(cond),
                                    Box::new(conds.remove(0)))
                            }

                            state.push(FsmState::Block(vec![
                                ast::Seq::If(
                                    cond,
                                    ast::SeqBlock(content),
                                    None,
                                )
                            ]));
                            true
                        } else {
                            false
                        }
                    //}
                } else {
                    false
                };

                // Transition into the second state after the first iteration.
                if !next.is_empty() || merged_prev_block {
                    initial.push(ast::Seq::Set(ast::Ident("FSM".to_string()), ast::Expr::Num(state_start)));
                    state_match.push(state_start);
                    state_start += 1;
                }

                // Include last loop entry as predecessor.
                if !next.is_empty() {
                    state.push(FsmState::Block(vec![
                        ast::Seq::If(
                            ast::Expr::Arith(ast::Op::Eq,
                                Box::new(ast::Expr::Ref(ast::Ident("FSM".to_string()))),
                                Box::new(ast::Expr::Num(state_start - 1))),
                            ast::SeqBlock(next),
                            None,
                        )
                    ]));
                }

                // Add our loop sequence.
                state.push(FsmState::Loop(expr, initial));
            }
        }
    }

    process(&mut output, &state_match, state, true);

    ast::Seq::Match(
        ast::Expr::Ref(ast::Ident("FSM".to_owned())),
        output.into_iter().map(|(x, block)| {
            (x.iter().map(|x| ast::Expr::Num(*x as i32)).collect(), ast::SeqBlock(block))
        }).collect())
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
