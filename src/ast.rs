#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ident(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Dir {
    In,
    Out,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Module(pub Vec<(Ident, Dir)>, pub Vec<Decl>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Edge {
    Pos,
    Neg,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EdgeRef(pub String, pub Edge);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Decl {
    Reg(Ident, Expr),
    RegArray(Ident, Expr, Expr),
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
    Slice(Ident, Box<Expr>, Option<Box<Expr>>),
    Concat(Vec<Expr>),
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Num(i64),
}
