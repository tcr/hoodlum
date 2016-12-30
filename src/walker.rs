use std::collections::HashMap;
use ast;

pub trait Walker {
    fn code(&mut self, _: &ast::Code) { }
    fn decl(&mut self, _: &ast::Decl) { }
    fn edgeref(&mut self, _: &ast::EdgeRef) { }
    fn expr(&mut self, _: &ast::Expr) { }
    fn seq(&mut self, _: &ast::Seq) { }
    fn toplevel(&mut self, _: &ast::Toplevel) { }
}

pub trait Walkable {
    fn walk<W: Walker>(&self, walker: &mut W);
}

impl Walkable for ast::Code {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.code(self);
        for item in &self.0 {
            item.walk(walker);
        }
    }
}

#[allow(unused_variables)]
impl Walkable for ast::Toplevel {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.toplevel(self);
        match self {
            &ast::Toplevel::Entity(ref id, ref args) => {
                //id.walk(walker);
                //for arg in &args {
                //    arg.walk(walker);
                //}
            }
            &ast::Toplevel::Impl(ref id, ref body) => {
                //id.walk(walker);
                for decl in body {
                    decl.walk(walker);
                }
            }
        }
    }
}

impl Walkable for ast::EdgeRef {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.edgeref(self);
        //self.0
        //self.1
    }
}

impl Walkable for ast::Decl {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.decl(self);
        match *self {
            ast::Decl::On(ref edgeref, ref block) => {
                edgeref.walk(walker);
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
                cond.walk(walker);
                t.walk(walker);
                if let &Some(ref block) = f {
                    block.walk(walker);
                }
            }
            ast::Seq::Loop(ref body) => {
                body.walk(walker);
            }
            ast::Seq::While(ref cond, ref body) => {
                cond.walk(walker);
                body.walk(walker);
            }
            ast::Seq::Set(ref btype, ref id, ref value) => {
                //TODO btype
                //TODO id
                value.walk(walker);
            }
            _ => { }
        }
    }
}


impl Walkable for ast::Expr {
    fn walk<W: Walker>(&self, walker: &mut W) {
        walker.expr(self);
        //TODO
        //match *self {
        //    ast::Seq::If(ref cond, ref t, ref f) => {
        //        // TODO cond
        //        cond.walk(walker);
        //        t.walk(walker);
        //        if let &Some(ref block) = f {
        //            block.walk(walker);
        //        }
        //    }
        //    ast::Seq::Loop(ref body) => {
        //        body.walk(walker);
        //    }
        //    ast::Seq::While(ref cond, ref body) => {
        //        // TODO cond
        //        body.walk(walker);
        //    }
        //    _ => { }
        //}
    }
}


pub struct InitWalker {
    pub init: HashMap<ast::Ident, ast::Expr>,
}

impl InitWalker {
    pub fn new() -> InitWalker {
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
    pub yield_count: usize,
    fsm_transition_count: usize,
}

impl CountWalker {
    pub fn new() -> CountWalker {
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
