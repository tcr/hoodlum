//! Experimental rewriting of yield, await, async blocks.

use VerilogState;
use ast;

use std::mem;
use CountWalker;
use Walkable;
use std::collections::BTreeSet;

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
}

#[derive(Debug, Clone)]
struct FsmCase {
    current: Option<i32>,
    states: BTreeSet<i32>,
    body: Vec<ast::Seq>,
}

impl FsmCase {
    fn empty() -> FsmCase {
        FsmCase {
            current: None,
            states: BTreeSet::new(),
            body: vec![],
        }
    }

    fn current_state(&self) -> i32 {
        self.current.unwrap()
    }

    fn substates(&self) -> BTreeSet<i32> {
        self.states.clone()
    }

    fn all_states(&self) -> BTreeSet<i32> {
        let mut ret = self.substates();
        if let Some(current) = self.current {
            ret.insert(current);
        }
        ret
    }
}

struct FsmGlobal {
    counter: FsmId,
}

#[derive(Clone, Debug)]
enum Transition {
    Yield(i32, Option<i32>),
    Precede(BTreeSet<i32>),
}

fn invert_expr(expr: ast::Expr) -> ast::Expr {
    if let ast::Expr::Unary(ast::UnaryOp::Not, inner) = expr {
        *inner
    } else {
        ast::Expr::Unary(ast::UnaryOp::Not, Box::new(expr))
    }
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
            ast::Seq::Loop(ast::SeqBlock(body)) => {
                block.push(ast::Seq::While(ast::Expr::Num(1), ast::SeqBlock(expand_await(body))));
            }
            ast::Seq::While(cond, ast::SeqBlock(body)) => {
                block.push(ast::Seq::While(cond, ast::SeqBlock(expand_await(body))));
            }
            ast::Seq::If(cond, ast::SeqBlock(then), else_seq) => {
                block.push(ast::Seq::If(cond,
                    ast::SeqBlock(expand_await(then)),
                    if let Some(ast::SeqBlock(else_seq)) = else_seq {
                        Some(ast::SeqBlock(expand_await(else_seq)))
                    } else {
                        None
                    }));
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

fn yield_count(seq: &ast::Seq) -> usize {
    let mut count = CountWalker::new();
    seq.walk(&mut count);
    count.yield_count
}


fn fsm_span(global: &mut FsmGlobal, base_state: FsmId, after: FsmCase, mut body: Vec<ast::Seq>, transition: Transition) -> (FsmCase, Vec<FsmCase>) {
    println!("\n!!! span AFTER {:?}\n", after);

    // Base conditions.
    let other_cases = vec![];
    let mut case = FsmCase::empty();

    // Iterate span from last sequence to first. For all non-loop items, we can
    // simply add them to the body. Once we reach a yielding structure, we decompose
    // this span into multiple spans.
    while let Some(seq) = body.pop() {
        match seq {
            ast::Seq::Loop(..) |
            ast::Seq::While(..) |
            ast::Seq::If(..) => {
                // Check to see if this is a yielding if { ... } block. If it
                // isn't, just insert it into the case. If it is, continue with
                // same logic as while/loop.
                if let ast::Seq::If(..) = seq {
                    if yield_count(&seq) == 0 {
                        case.body.insert(0, seq);
                        continue;
                    }

                    if let ast::Seq::If(_, _, Some(..)) = seq {
                        assert!(false, "there should not be an else clause in yielding if (yet)")
                    }
                }

                // Parse the content "following" this structure (which we already
                // walked through in this function) as its own span, using the
                // "after" content passed into this function as its after content.
                let following = mem::replace(&mut case.body, vec![]);
                let mut following_transition = transition.clone();
                if let Transition::Precede(ref mut next) = following_transition {
                    // Add global counter state, which is the last state a
                    // "following" section might leave us in.
                    next.insert(global.counter.value());
                };
                //let following_id = global.counter;
                println!("INPUT {:?} --> {:?}", after, following_transition);
                let (mut case, mut other_cases) = fsm_span(global, base_state, after, following, following_transition);
                println!("STRUCT {:?} {:?}", base_state, case);
                //let mut case = case.expect("missing a case");

                // We now have a case with our merged "after" and "following" content.
                // Now parse the yielding sequence with this case as its after content.
                let (structure, other) = fsm_structure(global, base_state, case.clone(), seq, body.len() == 0);
                case.states.extend(&structure.all_states());
                mem::replace(&mut case.body, structure.body);
                other_cases.extend(other);

                // Finally, parse the remaining span content preceding this yielding
                // sequence as its own span, with the new case as its after content.
                assert!(case.all_states().len() > 0);
                let mut targets = btreeset![base_state.value()];
                if let Some(current) = case.current {
                    targets.insert(current);
                }
                let next_transition = Transition::Precede(targets);
                //let next_transition = Transition::Precede(case.all_states());
                println!("BASE {:?} ----> {:?}", base_state, case);
                let (preceding, other) = fsm_span(global, base_state, case.clone(), body, next_transition);
                case.states.extend(preceding.all_states());
                mem::replace(&mut case.body, preceding.body);
                other_cases.extend(other);

                return (case, other_cases);
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
    // a conditional preceding the following content. If we are yielding, we
    // can just insert our content directly.
    match transition {
        Transition::Precede(targets) => {
            if targets.iter().find(|x| **x == 4).is_some() {
                println!("\n\nWOW {:?}\n\n", targets);
            }

            assert!(after.body.len() > 0, "when do we have precede without content?");

            // Get our inner, preceding content.
            let mut inner = mem::replace(&mut case.body, vec![]);

            // Only include a case if content actually exists.
            if inner.len() > 0 {
                let mut states = targets.clone();
                states.insert(base_state.value());

                let n = after.all_states();
                // TODO this is weird logic to make rewrite_await8 work
                if n.len() > 1 && targets.len() > 1 {
                    inner.push(ast::Seq::FsmTransition(base_state.value() as u32));
                }

                case.body.push(ast::Seq::If(ast::Expr::FsmEq(states),
                    ast::SeqBlock(inner),
                    None));
            }

            // Insert "after" content.
            //TODO assert all_states is subset of targets
            case.states.extend(after.all_states());
            //case.states.extend(targets);
            case.body.extend(after.body);
        }
        Transition::Yield(target, source) => {
            assert!(after.body.len() == 0, "when do we have yield with content?");

            let source_id = if let Some(source) = source {
                source
            } else {
                let id = global.counter.value();
                global.counter.incr();
                id
            };

            case.body.push(ast::Seq::FsmTransition(target as u32));
            case.current = Some(source_id);
        }
    }

    (case, other_cases)
}

/// Converts a sequence Loop, While, or If statement into a set of cases.
fn fsm_structure(global: &mut FsmGlobal, base_state: FsmId, after: FsmCase, seq: ast::Seq, is_first: bool) -> (FsmCase, Vec<FsmCase>) {
    println!("\n!!! structure AFTER {:?}\n", after);


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
            if !has_loop && !is_if {
                assert!(spans.len() > 1, "loop statements require one yield");
            }

            let first = spans.remove(0);
            let last = spans.pop();

            // TODO does base_State differ from id ever
            let id = if !is_first || match last {
                Some(ref body) => body.len() > 0,
                _ => false
            } {
                let id = global.counter;
                global.counter.incr();
                id
            } else {
                base_state
            };

            println!("TIM\n\n{:?} oh {:?}\n\n", id, base_state);

            // Generate intermediate cases (from everything but the first and last case)
            let mut inner_cases = vec![];
            let mut last_id = id;
            for span in spans.into_iter().rev() {
                // Parse this span as its own content.
                let intermediate_id = global.counter;
                let (case, span_cases) = fsm_span(global, intermediate_id, FsmCase::empty(), span, Transition::Yield(last_id.value(), None));
                last_id = FsmId(case.current_state());
                inner_cases.push(case);
                inner_cases.extend(span_cases);
            }
            // Order our cases according to their input.
            inner_cases.reverse();

            // Parse the first block.
            let (first_block, first_cases) = fsm_span(global, base_state, FsmCase::empty(), first, Transition::Yield(last_id.value(), Some(id.value())));
            println!("F I R S T: {:?}", first_block);

            let mut case = FsmCase::empty();
            case.current = Some(id.value());

            // Generate a state whitelist.
            let mut state_whitelist = btreeset![];
            if !is_if {
                state_whitelist.insert(id.value());
            }
            state_whitelist.insert(base_state.value());

            // Generate initial "first" case. Output depends on if we have a
            // condition (if/while) or not (loop).
            if let Some(mut cond) = cond {
                // If our first case has content, generate an if {...} else {...}
                // loop. Otherwise, invert the conditional, just generate an
                // if !cond {...}
                if !is_if {
                    for item in first_block.all_states() {
                        state_whitelist.insert(item);
                    }
                }

                // Check if we can invert our expression.
                assert!(first_block.body.len() > 0);
                if is_first && first_block.body.len() == 1 && matches!(first_block.body[0], ast::Seq::FsmTransition(..)) {
                    // Expand our condition to also check our FSM states.
                    cond = invert_expr(cond);
                    if !(is_first && after.substates().len() == 0) || is_if {
                        cond = ast::Expr::Arith(ast::Op::Or, Box::new(ast::Expr::FsmNe(state_whitelist)), Box::new(cond));
                    }

                    let seq = ast::Seq::If(cond,
                        ast::SeqBlock(after.body),
                        None);
                    case.states.extend(first_block.substates());
                    case.body.push(seq);
                } else {
                    // Expand our condition to also check our FSM states.
                    if !(is_first && after.substates().len() == 0) || is_if {
                        cond = ast::Expr::Arith(ast::Op::And, Box::new(ast::Expr::FsmEq(state_whitelist)), Box::new(cond));
                    }

                    let seq = ast::Seq::If(cond,
                        ast::SeqBlock(first_block.body.clone()),
                        Some(ast::SeqBlock(after.body)));
                    case.states.extend(first_block.substates());
                    case.body.push(seq);
                }
            } else {
                case.states.extend(first_block.substates());
                case.body.extend(first_block.body.clone());

                // TODO refactor this logic
                // see rewrite_fsm_while_4
                if base_state != id && first_block.states.len() > 1 {
                    let seq = ast::Seq::If(ast::Expr::FsmEq(btreeset![base_state.value()]),
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
                assert!(has_loop || is_if);
                let mut other_cases = vec![];
                other_cases.extend(first_cases);
                other_cases.extend(inner_cases);
                return (case, other_cases);
            }

            // If we have a "last" block, it must precede our loop condition. We
            // use our generated loop construct as its "after" condition to ensure
            // our state matching generation is consistent.
            let last = last.expect("missing last span");
            let mut targets = btreeset![];
            targets.insert(id.value());
            //targets.extend(case.all_states());
            let (mut last_block, last_cases) = fsm_span(global, id, case.clone(), last, Transition::Precede(targets));
            last_block.states.extend(case.all_states());

            let mut other_cases = vec![];
            other_cases.extend(first_cases);
            other_cases.extend(inner_cases);
            other_cases.extend(last_cases);

            (last_block, other_cases)
        }
        _ => {
            panic!("expected structure");
        }
    }
}




/// Returns an ast::Seq::Match containing our FSM.
pub fn fsm_rewrite(input: &ast::Seq, v: &VerilogState) -> (ast::Seq, VerilogState) {
    // Extract body from FSM sequence.
    let mut body = if let &ast::Seq::Async(ast::SeqBlock(ref body)) = input {
        body.clone()
    } else {
        panic!("Cannot transform non-FSM sequence.");
    };

    // Our FSM is structured as a loop { ...; yield; } statement.
    body.push(ast::Seq::Yield);
    let loop_seq = ast::Seq::Loop(ast::SeqBlock(body));

    // Generate base loop structure.
    let mut global = FsmGlobal {
        counter: FsmId(0)
    };
    global.counter.incr();
    let (case, mut cases) = fsm_structure(&mut global, FsmId(0), FsmCase::empty(), loop_seq, true);
    cases.insert(0, case);

    println!("cases {:?}", cases);

    // Verify output cases.
    {
        let mut state_check = btreeset![];
        for case in &cases {
            for state in case.all_states() {
                if state_check.contains(&state) {
                    panic!("duplicate state {:?} in output.", state);
                }
                state_check.insert(state);
            }
        }
    }

    // Generate match cases from our case output.
    let mut output: Vec<(Vec<i32>, ast::SeqBlock)> = vec![];
    for case in cases {
        output.push((
            case.all_states().into_iter().collect(),
            ast::SeqBlock(case.body)));
    }
    (ast::Seq::FsmCase(output), v.clone())
}
