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
pub struct Entity(pub Ident, pub Vec<(Ident, Dir)>, pub Vec<Decl>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Edge {
    Pos,
    Neg,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EdgeRef(pub Ident, pub Edge);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decl {
    Reg(Ident, Option<Expr>),
    RegArray(Ident, Expr, Expr),
    Let(Ident, Ident, Vec<Ident>),
    On(EdgeRef, SeqBlock),
    Always(CombBlock),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeqBlock(pub Vec<Seq>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Seq {
    If(Expr, SeqBlock, Option<SeqBlock>),
    Reset(Ident, SeqBlock),
    Set(Ident, Expr),
    Match(Expr, Vec<(Vec<Expr>, SeqBlock)>),

    While(Expr, SeqBlock),
    Loop(SeqBlock),
    Fsm(SeqBlock),
    Yield,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CombBlock(pub Vec<Comb>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Comb {
    If(Expr, CombBlock, Option<CombBlock>),
    Assign(Ident, Expr)
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    And,
    Or,
    Lt,
    Gt,
    Ne,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UnaryOp {
    Not,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Slice(Ident, Box<Expr>, Option<Box<Expr>>),
    Concat(Vec<Expr>),
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Num(i32),
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
