use VerilogState;
use ast;

use itertools::Itertools;
use std::mem;
use CountWalker;
use Walkable;

fn invert_expr(expr: ast::Expr) -> ast::Expr {
    if let ast::Expr::Unary(ast::UnaryOp::Not, inner) = expr {
        *inner
    } else {
        ast::Expr::Unary(ast::UnaryOp::Not, Box::new(expr))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct FsmId(i32);

impl FsmId {
    fn value(&self) -> i32 {
        self.0
    }

    fn incr(&mut self) -> FsmId {
        self.0 += 1;
        FsmId(self.0)
    }

    fn decr(&mut self) -> FsmId {
        self.0 -= 1;
        FsmId(self.0)
    }
}

fn fsm_match_list(op: ast::Op, list: &[i32]) -> ast::Expr {
    assert!(list.len() > 0);

    let mut list = list.to_vec();
    list.reverse();

    let mut cond = ast::Expr::Arith(op.clone(),
        Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
        Box::new(ast::Expr::FsmValue(list[0] as u32)));
    for item in &list[1..] {
        cond = ast::Expr::Arith(match op {
            ast::Op::Eq => ast::Op::Or,
            _ => ast::Op::And,
        },
            Box::new(ast::Expr::Arith(op.clone(),
                Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
                Box::new(ast::Expr::FsmValue(*item as u32)))),
            Box::new(cond));
    }
    cond
}

#[derive(Debug, Clone)]
struct FsmCase {
    states: Vec<i32>,
    body: Vec<ast::Seq>,
}

fn expand_await(body: Vec<ast::Seq>) -> Vec<ast::Seq> {
    // Expand and normalize sequences "await" and "loop".
    let mut block = vec![];
    for item in body {
        match item {
            ast::Seq::Await(ref cond) => {
                block.push(ast::Seq::Yield);
                block.push(ast::Seq::While(invert_expr(cond.clone()),
                        ast::SeqBlock(vec![ast::Seq::Yield])));
            }
            // TODO this is a temporary hack
            ast::Seq::Loop(body) => {
                block.push(ast::Seq::While(ast::Expr::Num(1), body));
            }
            _ => block.push(item.clone()),
        };
    }
    block
}

fn fsm_split_yield(body: Vec<ast::Seq>) -> Vec<Vec<ast::Seq>> {
    let mut cur = vec![];
    let mut ret = vec![];
    for item in body {
        match item {
            ast::Seq::Yield => {
                ret.push(mem::replace(&mut cur, vec![]));
            }
            _ => cur.push(item),
        }
    }
    ret.push(cur);
    ret
}

struct FsmGlobal {
    counter: FsmId,
}

#[derive(Clone, Debug)]
enum Transition {
    Yield(i32),
    Precede(Vec<i32>),
}

fn normalize<T: ::std::hash::Hash + Eq + Clone + Ord>(input: &[T]) -> Vec<T> {
    input.to_vec().into_iter().unique().sorted()
}

fn fsm_span(global: &mut FsmGlobal, base_state: FsmId, after: FsmCase, mut body: Vec<ast::Seq>, transition: Transition) -> (Option<FsmCase>, Vec<FsmCase>) {
    // Base conditions.
    let other_cases = vec![];
    let mut case = FsmCase {
        states: vec![],
        body: vec![]
    };

    // Terminate early for empty content.
    if body.is_empty() && matches!(transition, Transition::Precede(..)) && after.body.is_empty() {
        return (None, other_cases);
    }

    while let Some(seq) = body.pop() {
        match seq {
            ast::Seq::Loop(..) |
            ast::Seq::While(..) |
            ast::Seq::If(..) => {
                // Check to see if this is a structure if { ... } block.
                if let ast::Seq::If(..) = seq {
                    let mut count = CountWalker::new();
                    seq.walk(&mut count);
                    if count.yield_count == 0 {
                        case.body.insert(0, seq);
                        continue;
                    }

                    if let ast::Seq::If(_, _, Some(..)) = seq {
                        assert!(false, "there should not be an else clause in yielding if (yet)")
                    }
                }

                // Parse the "following" content as its own span.
                let following = mem::replace(&mut case.body, vec![]);
                println!("--------> {:?} {:?}", transition, base_state);
                let following_transition = match transition {
                    Transition::Precede(ref next) => {
                        let mut n = next.clone();
                        n.push(global.counter.value());
                        Transition::Precede(n)
                    }
                    Transition::Yield(..) => {
                        transition.clone()
                    }
                };
                let following_id = global.counter;
                let (case, mut other_cases) = fsm_span(global, following_id, after, following, following_transition);
                let mut case = case.expect("missing a case");

                // Parse loop with our merged "after" and "following" blocks.
                println!("STRUCT {:?} {:?}", base_state, case);
                let (structure, other) = fsm_structure(global, base_state, case.clone(), seq);
                case.states.extend(&structure.states);
                mem::replace(&mut case.body, structure.body);
                other_cases.extend(other);

                // Parse the remaining content in "body" as its own span.
                assert!(case.states.len() > 0);
                let next_transition = Transition::Precede(vec![base_state.value()]);
                if let (Some(preceding), other) = fsm_span(global, base_state, case.clone(), body, next_transition) {
                    case.states.extend(preceding.states);
                    mem::replace(&mut case.body, preceding.body);
                    other_cases.extend(other);
                }

                return (Some(case), other_cases);
            }
            ast::Seq::Yield => {
                panic!("expected sequence not yield")
            }
            _ => {
                case.body.insert(0, seq);
            }
        }
    }

    // If we have content following this span, we want to make our current span
    // a conditional preceding the following content.
    if after.body.len() > 0 {
        // Get our inner, preceding content.
        let mut inner = mem::replace(&mut case.body, vec![]);

        // Only include a case if content actually exists.
        if inner.len() > 0 {
            match transition {
                Transition::Yield(target) => {
                    let id = global.counter;
                    global.counter.incr();

                    inner.push(ast::Seq::FsmTransition(target as u32));
                    case.body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &[id.value()]),
                        ast::SeqBlock(inner),
                        None));
                    case.states.push(id.value());

                }
                Transition::Precede(targets) => {
                    let mut states = targets.clone();
                    states = normalize(&states);

                    let n = normalize(&after.states);
                    // TODO this is weird logic to make rewrite_await8 work
                    println!("~~~~~~~~> {:?} {:?}", n, targets);
                    if n.len() > 1 && targets.len() > 1 {
                        inner.push(ast::Seq::FsmTransition(*n.last().unwrap() as u32));
                    }

                    case.body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &states),
                        ast::SeqBlock(inner),
                        None));
                    //TODO?
                    case.states.extend(targets);

                }
            }
        }
        case.body.extend(after.body);
    } else {
        match transition {
            Transition::Yield(target) => {
                let id = global.counter;
                global.counter.incr();

                case.body.push(ast::Seq::FsmTransition(target as u32));
                case.states.push(id.value());
            }
            Transition::Precede(..) => {
                // ignore
            }
        }
    }

    (Some(case), other_cases)
}

/// Converts a sequence Loop, While, or If statement into a set of cases.
fn fsm_structure(global: &mut FsmGlobal, base_state: FsmId, after: FsmCase, seq: ast::Seq) -> (FsmCase, Vec<FsmCase>) {
    let id = global.counter;
    global.counter.incr();

    match seq {
        ast::Seq::Loop(..) |
        ast::Seq::While(..) |
        ast::Seq::If(..) => {
            // Extract the conditional.
            let (cond, mut body, is_if) = match seq {
                ast::Seq::Loop(ast::SeqBlock(body)) => (None, body, false),
                ast::Seq::While(cond, ast::SeqBlock(body)) => (Some(cond), body, false),
                ast::Seq::If(cond, ast::SeqBlock(body), _) => (Some(cond), body, true),
                _ => unreachable!(),
            };

            // Expand await statements. Split into yield blocks.
            body = expand_await(body);
            let has_loop = if let Some(pos) = body.iter().position(|x| matches!(*x, ast::Seq::Loop(..))) {
                body.split_off(pos + 1);
                true
            } else {
                false
            };
            let mut spans = fsm_split_yield(body);
            if !has_loop {
                assert!(spans.len() > 1, "loop statements require one yield");
            }

            let first = spans.remove(0);
            let last = spans.pop();

            // Generate intermediate cases (from everything but the first and last case)
            let mut inner_cases = vec![];
            let mut last_id = id;
            for span in spans.into_iter().rev() {
                // Parse this span as its own content.
                let intermediate_id = global.counter;
                let (case, span_cases) = fsm_span(global, intermediate_id, FsmCase {
                    body: vec![],
                    states: vec![]
                }, span, Transition::Yield(last_id.value()));
                if let Some(case) = case {
                    last_id = FsmId(case.states[0]);
                    inner_cases.push(case);
                }
                inner_cases.extend(span_cases);
            }

            // Decrease global counter?
            //TODO explain this
            global.counter.decr();
            // Parse the first block.
            println!("FIRST {:?} {:?}", id, base_state);
            let (first_block, first_cases) = fsm_span(global, id, FsmCase {
                body: vec![],
                states: vec![]
            }, first, Transition::Yield(last_id.value()));

            let mut case = FsmCase {
                states: vec![id.value()],
                body: vec![],
            };

            // Generate a state whitelist.
            let mut state_whitelist = vec![];
            if !is_if {
                state_whitelist.push(id.value());
            }
            state_whitelist.push(base_state.value());
            state_whitelist = normalize(&state_whitelist);

            // Generate initial "first" case. Output depends on if we have a
            // condition (if/while) or not (loop).
            if let Some(mut cond) = cond {
                // If our first case has content, generate an if {...} else {...}
                // loop. Otherwise, invert the conditional, just generate an
                // if !cond {...}
                if let Some(FsmCase {
                    states: first_states,
                    body: first_body,
                }) = first_block {
                    // Expand our condition to also check our FSM states.
                    cond = ast::Expr::Arith(ast::Op::And, Box::new(fsm_match_list(ast::Op::Eq, &state_whitelist)), Box::new(cond));

                    let seq = ast::Seq::If(cond,
                        ast::SeqBlock(first_body),
                        Some(ast::SeqBlock(after.body)));
                    case.states.extend(&first_states[1..]);
                    case.body.push(seq);
                } else {
                    // Expand our condition to also check our FSM states.
                    cond = ast::Expr::Arith(ast::Op::And, Box::new(fsm_match_list(ast::Op::Eq, &state_whitelist)), Box::new(cond));

                    // We don't have any else case, so jump straight in.
                    let seq = ast::Seq::If(invert_expr(cond),
                        ast::SeqBlock(after.body),
                        None);
                    case.body.push(seq);
                }
            } else {
                let first_case = first_block.expect("Lacking first case in loop.");
                case.states.extend(&first_case.states[1..]);
                case.body.extend(first_case.body.clone());

                // TODO remove this logic
                // see rewrite_fsm_while_4
                if base_state != id && first_case.states.len() > 1 {
                    let seq = ast::Seq::If(fsm_match_list(ast::Op::Eq, &[base_state.value()]),
                        ast::SeqBlock(vec![
                            ast::Seq::FsmTransition(id.value() as u32),
                        ]),
                        None);
                    case.body.insert(0, seq);
                }
            }

            // If we are in a loop, we don't have a "last" case. Add our other
            // cases and exit early.
            if last.is_none() {
                assert!(has_loop);
                let mut other_cases = vec![];
                other_cases.extend(first_cases);
                other_cases.extend(inner_cases);
                return (case, other_cases);
            }

            // If we have a "last" block, it must precede our loop condition. We
            // use our generated loop construct as its "after" condition to ensure
            // our state matching generation is consistent.
            let last = last.expect("missing last span");
            let (last_block, last_cases) = fsm_span(global, id, case.clone(), last, Transition::Precede(vec![id.value()]));
            if let Some(mut last_block) = last_block {
                last_block.states.extend(case.states);

                let mut other_cases = vec![];
                other_cases.extend(first_cases);
                other_cases.extend(inner_cases);
                other_cases.extend(last_cases);

                (last_block, other_cases)
            } else {
                let mut other_cases = vec![];
                other_cases.extend(first_cases);
                other_cases.extend(inner_cases);
                (case, other_cases)
            }
        }
        _ => {
            panic!("expected structure");
        }
    }
}




/// Returns an ast::Seq::Match containing our FSM.
pub fn fsm_rewrite(input: &ast::Seq, v: &VerilogState) -> (ast::Seq, VerilogState) {
    // Extract body from FSM sequence.
    let mut body = if let &ast::Seq::Fsm(ast::SeqBlock(ref body)) = input {
        body.clone()
    } else {
        panic!("Cannot transform non-FSM sequence.");
    };

    // Our FSM is structured as a loop { ...; yield; } statement.
    body.push(ast::Seq::Yield);
    let loop_seq = ast::Seq::Loop(ast::SeqBlock(body));

    let mut global = FsmGlobal {
        counter: FsmId(0)
    };
    let (case, mut cases) = fsm_structure(&mut global, FsmId(0), FsmCase {
        body: vec![],
        states: vec![],
    }, loop_seq);
    cases.insert(0, case);

    println!("cases {:?}", cases);

    // Match cases
    let mut output: Vec<(Vec<ast::Expr>, ast::SeqBlock)> = vec![];
    for case in cases {
        output.push((
            case.states.iter().unique().into_iter().sorted().into_iter().map(|x| ast::Expr::Num(*x)).collect(),
            ast::SeqBlock(case.body)));
    }

    (ast::Seq::Match(ast::Expr::Ref(ast::Ident("_FSM".to_string())),
        output), v.clone())
}
