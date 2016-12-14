use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Code(pub Vec<Entity>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    In,
    Out,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub Ident, pub Vec<(Ident, Dir, Option<i32>)>, pub Vec<Decl>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Edge {
    Pos,
    Neg,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EdgeRef(pub Ident, pub Edge);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decl {
    Latch(Ident, Vec<Expr>),
    Reg(Ident, Vec<Expr>, Option<Expr>),
    Let(Ident, Ident, Vec<(Ident, Expr)>),
    Const(Ident, Expr),
    On(EdgeRef, SeqBlock),
    Always(SeqBlock),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeqBlock(pub Vec<Seq>);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlockType {
    Blocking,
    NonBlocking,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Seq {
    If(Expr, SeqBlock, Option<SeqBlock>),
    Set(BlockType, Ident, Expr),
    SetIndex(BlockType, Ident, Expr, Expr),
    SetRange(BlockType, Ident, Expr, Expr, Expr),
    Match(Expr, Vec<(Vec<Expr>, SeqBlock)>),

    While(Expr, SeqBlock),
    Loop(SeqBlock),
    Async(SeqBlock),
    FsmCaseTransition(Ident),
    Fsm(Vec<(Ident, SeqBlock)>),
    Yield,
    Await(Expr),

    FsmCase(Vec<(Vec<i32>, SeqBlock)>),
    FsmTransition(u32),
}

//#[derive(Clone, Debug, Eq, Hash, PartialEq)]
//pub struct CombBlock(pub Vec<Comb>);
//
//#[derive(Clone, Debug, Eq, Hash, PartialEq)]
//pub enum Comb {
//    If(Expr, CombBlock, Option<CombBlock>),
//    Assign(Ident, Expr),
//}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lte,
    Gte,
    And,
    Or,
    Lt,
    Gt,
    Ne,
    BinOr,
    BinAnd,
    LShift,
    RShift,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Not,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Slice(Ident, Box<Expr>, Option<Box<Expr>>),
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
    Concat(Vec<Expr>),
    Repeat(Box<Expr>, Box<Expr>),
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Num(i32),
    Placeholder,

    FsmEq(BTreeSet<i32>),
    FsmNe(BTreeSet<i32>),
}

impl Expr {
    pub fn to_i32(&self) -> i32 {
        match *self {
            Expr::Num(value) => value,
            _ => {
                panic!("Called to_i32 on non-Num.")
            }
        }
    }
}
