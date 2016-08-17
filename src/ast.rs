#[derive(Debug, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub enum Dir {
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct Module(pub Vec<(Ident, Dir)>, pub Vec<Decl>);

#[derive(Debug, Clone)]
pub enum Edge {
    Pos,
    Neg,
}

#[derive(Debug, Clone)]
pub struct EdgeRef(pub String, pub Edge);

#[derive(Debug, Clone)]
pub enum Decl {
    Reg(Ident),
    RegArray(Ident, Expr),
    On(EdgeRef, SeqBlock),
    Always(CombBlock),
}

#[derive(Debug, Clone)]
pub struct SeqBlock(pub Vec<Seq>);

#[derive(Debug, Clone)]
pub enum Seq {
    If(Expr, SeqBlock, Option<SeqBlock>),
    Set(Ident, Expr),
}

#[derive(Debug, Clone)]
pub struct CombBlock(pub Vec<Comb>);

#[derive(Debug, Clone)]
pub enum Comb {
    If(Expr, CombBlock, Option<CombBlock>),
    Assign(Ident, Expr)
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Slice(Ident, Box<Expr>, Option<Box<Expr>>),
    Concat(Vec<Expr>),
    Ref(Ident),
    Arith(Op, Box<Expr>, Box<Expr>),
    Num(i64),
}
