use VerilogState;
use ast;

use itertools::Itertools;
use std::mem;
use std::collections::HashMap;

/*

fsm {
    a <= 1;
    yield;
    a <= 2;
    yield;
    while !result {
        a <= 3;
        yield;
    }
    a <= 4;
}
*/

fn invert_expr(expr: ast::Expr) -> ast::Expr {
    if let ast::Expr::Unary(ast::UnaryOp::Not, inner) = expr {
        *inner
    } else {
        ast::Expr::Unary(ast::UnaryOp::Not, Box::new(expr))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct FsmId(u32);

impl FsmId {
    fn next(&self) -> FsmId {
        FsmId(self.0 + 1)
    }
}

#[derive(Clone, Debug)]
enum FsmTransition {
    Yield(FsmId),
    Goto(FsmId),
    While {
        cond: ast::Expr,
        then: FsmId,
        after: FsmId,
    },
    If {
        cond: ast::Expr,
        then: FsmId,
        otherwise: FsmId
    },
}

#[derive(Clone, Debug)]
struct FsmState {
    body: Vec<ast::Seq>,
    transition: FsmTransition,
}

fn fsm_sequence(states: &mut HashMap<FsmId, FsmState>, initial: FsmId, input: &[ast::Seq]) -> FsmId {
    use self::FsmTransition::*;

    // Current state is set to Goto our initial state.
    let mut cur: (FsmId, FsmState) = (initial.next(), FsmState {
        body: vec![],
        transition: Goto(initial),
    });

    fn next_state(states: &mut HashMap<FsmId, FsmState>, cur: &mut (FsmId, FsmState), transition: FsmTransition) {
        // Create a new state we will work with that yields to current state.
        let next_cur = (cur.0.next(), FsmState {
            body: vec![],
            transition: transition,
        });

        // Swap our last and next states. Add last state to state list.
        let last_cur = mem::replace(cur, next_cur);
        states.insert(last_cur.0, last_cur.1);
    }

    // Expand and normalize sequences "await" and "loop".
    let mut block = vec![];
    for item in input {
        match item {
            &ast::Seq::Loop(ref body) => {
                block.push(item.clone());

                // After we reach a loop, we have no further statements. Break.
                break;
            }
            &ast::Seq::Await(ref cond) => {
                block.push(ast::Seq::Yield);
                block.push(ast::Seq::While(invert_expr(cond.clone()),
                        ast::SeqBlock(vec![ast::Seq::Yield])));
            }
            _ => {
                block.push(item.clone());
            }
        };
    }

    // Iterate over our normalized input to generate new states.
    for item in block.into_iter().rev() {
        match item {
            ast::Seq::Loop(..) |
            ast::Seq::While(..) => {
                // While loops must contain a yield statement. Given this,
                // we decompose the while loop into several states:
                //
                // {A}
                // while cond {
                //   {B}
                //   yield;
                //   {C}
                // }
                // {D}
                //
                // Becomes:
                //
                // 1: {A}, Goto(3↓)
                // 2: {C}, Goto(3↓)
                // 3: {empty}, While { cond, then: 4↓, after: 5↓ }
                // 4: {B}, Yield(2↑)
                // 5: {D}, ...

                // Cache some states.
                let first_after_state = cur.0;
                let last_inner_state = cur.0.next();

                let (cond, body) = match item {
                    ast::Seq::Loop(ast::SeqBlock(body)) => (None, body),
                    ast::Seq::While(cond, ast::SeqBlock(body)) => (Some(cond), body),
                    _ => unreachable!(),
                };

                // Process our inner content to generate states 4 and 2.
                let first_inner_state = fsm_sequence(states, first_after_state, &body);

                // Commit the current state (state 5).
                let while_state;
                if let Some(cond) = cond {
                    let transition = While {
                        cond: cond,
                        then: first_inner_state,
                        after: first_after_state,
                    };
                    next_state(states, &mut cur, transition);
                    cur.0 = first_inner_state.next();
                    while_state = cur.0;
                } else {
                    let transition = Goto(first_inner_state);
                    next_state(states, &mut cur, transition);
                    cur.0 = first_inner_state.next();
                    while_state = first_inner_state;
                };

                // Modify the last state to transition of the while loop to
                // jump back to the beginning of the while loop.
                if let Some(state) = states.get_mut(&last_inner_state) {
                    match state.transition {
                        Yield(ref mut target) |
                        Goto(ref mut target) => {
                            *target = while_state;
                        }
                        _ => {
                            unreachable!("cannot loop this thing");
                        }
                    }
                }

                // Commit the current state. We Goto the beginning of the
                // while loop, because the while state must have an empty body.
                //cur.0 = while_state;
                //cur.1.transition = Goto(while_state);
                //next_state(states, &mut cur, transition);
            }
            ast::Seq::Yield => {
                // If we have an empty Goto statement, we just overwrite the current state.
                if cur.1.body.is_empty() && match cur.1.transition {
                    Goto(..) => true,
                    _ => false
                } {
                    if let Goto(state) = cur.1.transition {
                        cur.1.transition = Yield(state);
                    }
                } else {
                    let transition = Yield(cur.0);
                    next_state(states, &mut cur, transition);
                }

                println!("CURRENT YIELD {:?}", cur);
            }
            seq => {
                cur.1.body.push(seq);
            }
        }
    }

    // Add final state.
    if cur.1.body.is_empty() && match cur.1.transition { Goto(..) => true, _ => false } {
        (cur.0).0 -= 1;
    } else {
        states.insert(cur.0, cur.1);
    }

    // Check all states to ensure there are no empty-body Goto statements.
    //for (key, state) in states {
    //    if let Goto(..) = state.transition {
    //        assert!(!state.body.is_empty(), "created an empty goto statement");
    //    }
    //}

    // Return new highest id.
    cur.0
}

fn fsm_match_list(op: ast::Op, list: &[u32]) -> Option<ast::Expr> {
    if list.len() == 0 {
        return None;
    }

    let mut cond = ast::Expr::Arith(op.clone(),
        Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
        Box::new(ast::Expr::FsmValue(list[0])));
    for item in &list[1..] {
        cond = ast::Expr::Arith(match op {
            ast::Op::Eq => ast::Op::Or,
            _ => ast::Op::And,
        },
            Box::new(ast::Expr::Arith(op.clone(),
                Box::new(ast::Expr::Ref(ast::Ident("_FSM".to_string()))),
                Box::new(ast::Expr::FsmValue(*item)))),
            Box::new(cond));
    }
    Some(cond)
}

fn fsm_compose(id: FsmId, states: &HashMap<FsmId, FsmState>, mut ids: Vec<i32>, mut body: Vec<ast::Seq>, mut next: Vec<FsmId>) -> (Vec<i32>, Vec<ast::Seq>, Vec<FsmId>) {
    use self::FsmTransition::*;

    let state = states.get(&id).unwrap();
    ids.push(id.0 as i32);

    match &state.transition {
        &FsmTransition::Goto(next_id) => {
            if !state.body.is_empty() {
                body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &[id.0 as u32]).unwrap(),
                    ast::SeqBlock(state.body.clone()),
                    None));
            }

            return fsm_compose(next_id, states, ids, body, next);
        }
        &FsmTransition::If { .. } => {
            // TODO
        }
        &FsmTransition::While { ref cond, then, after } => {
            let (then_ids, then_body, then_next) = fsm_compose(then, states, ids, vec![], next);
            ids = then_ids;
            next = then_next;

            let (after_ids, after_body, after_next) = fsm_compose(after, states, ids, vec![], next);
            ids = after_ids;
            next = after_next;

            body.push(ast::Seq::If(cond.clone(),
                ast::SeqBlock(then_body),
                Some(ast::SeqBlock(after_body))));
        }
        &FsmTransition::Yield(next_id) => {
            body.extend(state.body.clone());

            body.push(ast::Seq::FsmTransition(next_id.0));
            next.push(next_id);
        }
    }

    (ids, body, next)
}

fn fsm_flip_ids(states: &mut HashMap<FsmId, FsmState>, max: u32) {
    let mut flipped = HashMap::new();
    for (key, state) in states.iter() {
        let mut s = state.clone();
        match s.transition {
            FsmTransition::Goto(ref mut next_id) => {
                next_id.0 = max - next_id.0;
            }
            FsmTransition::If { ref cond, ref mut then, ref mut otherwise } => {
                then.0 = max - then.0;
                otherwise.0 = max - otherwise.0;
            }
            FsmTransition::While { ref cond, ref mut then, ref mut after } => {
                then.0 = max - then.0;
                after.0 = max - after.0;
            }
            FsmTransition::Yield(ref mut next_id) => {
                next_id.0 = max - next_id.0;
            }
        }
        flipped.insert(FsmId(max - key.0), s);
    }
    mem::replace(states, flipped);
}

/// Returns an ast::Seq::Match containing our FSM.
fn fsm_rewrite2(input: &ast::Seq, v: &VerilogState) -> (ast::Seq, VerilogState) {
    let initial_state = FsmId(0);

    let mut body = if let &ast::Seq::Fsm(ast::SeqBlock(ref body)) = input {
        body.clone()
    } else {
        panic!("Cannot transform non-FSM sequence.");
    };

    // FSM acts like a loop { ... yield; } block.
    body.push(ast::Seq::Yield);

    // Generate states.
    let mut states = HashMap::new();
    let max_id = fsm_sequence(&mut states, initial_state, &body);

    // Modify our final generated state.
    if let Some(ref mut state) = states.get_mut(&initial_state.next()) {
        match state.transition {
            FsmTransition::Yield(ref mut target) |
            FsmTransition::Goto(ref mut target) => {
                *target = max_id;
            }
            _ => {
                unreachable!("cannot loop this thing");
            }
        }
    }

    fsm_flip_ids(&mut states, max_id.0);

    // NOTE Dump them out.
    println!("\nDONE!!!!!!!!!!");
    println!("MAX_ID {:?}", max_id);
    let mut keys: Vec<_> = states.keys().collect();
    keys.sort();
    for key in keys.iter() {
        println!(" {:?}:   {:?}", key, states.get(key));
    }

    // Reconstitute states into a match statement. Start with the lowest state.
    let mut next = vec![FsmId(0)];
    let mut cases = vec![];
    let mut done = vec![];
    while next.len() > 0 {
        let an_id = next.remove(0);
        done.push(an_id);

        println!("hey");
        let (ids, body, new_next) = fsm_compose(an_id, &states, vec![], vec![], next);
        cases.push((ids.iter().map(|x| ast::Expr::Num(*x)).collect(), ast::SeqBlock(body)));
        next = new_next.into_iter().filter(|x| done.iter().find(|y| *x == **y).is_none()).collect();
    }

    (ast::Seq::Match(ast::Expr::Ref(ast::Ident("_FSM".to_string())),
        cases), v.clone())
}


#[derive(Debug)]
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
    counter: i32,
}

fn fsm_span(global: &mut FsmGlobal, mut body: Vec<ast::Seq>, after: Vec<ast::Seq>, transition: Option<i32>) -> (Option<FsmCase>, Vec<FsmCase>) {
    let mut other_cases = vec![];
    let mut case = FsmCase {
        states: vec![],
        body: vec![]
    };

    // Terminate early.
    if body.is_empty() && transition.is_none() {
        return (None, other_cases);
    }

    while let Some(seq) = body.pop() {
        match seq {
            ast::Seq::Loop(..) |
            ast::Seq::While(..) => {
                // Parse the remaining content as its own span.
                //TODO use "following"
                let following = mem::replace(&mut case.body, vec![]);
                let (mut case, mut other_cases) = fsm_span(global, following, after, transition);
                let mut case = case.unwrap();

                // Parse loop with provided "after" content.
                let (structure, other) = fsm_structure(global, case.body.clone(), seq);
                case.states.extend(structure.states);
                mem::replace(&mut case.body, structure.body);
                other_cases.extend(other);

                println!("1 {:?}", global.counter);
                if let (Some(preceding), other) = fsm_span(global, body, case.body.clone(), transition) {
                    case.states.extend(preceding.states);
                    mem::replace(&mut case.body, preceding.body);
                }
                println!("2");

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

    let id = global.counter;

    // Extract the inner body, prepend it as a conditional to this state.
    if !after.is_empty() {
        let mut inner = mem::replace(&mut case.body, vec![]);
        // Elide empty transition bodies
        if !inner.is_empty() {
            global.counter += 1;
            if transition.is_some() {
                inner.push(ast::Seq::FsmTransition(transition.unwrap() as u32));
            }
            case.body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &[id as u32]).unwrap(),
                ast::SeqBlock(inner),
                None));
            case.states.push(id);
        }
        case.body.extend(after);
    } else {
        if transition.is_some() {
            case.body.push(ast::Seq::FsmTransition(transition.unwrap() as u32));
        }
        global.counter += 1;
        case.states.push(id);
    }

    (Some(case), other_cases)
}


fn fsm_structure(global: &mut FsmGlobal, after: Vec<ast::Seq>, seq: ast::Seq) -> (FsmCase, Vec<FsmCase>) {
    let id = global.counter;
    global.counter += 1;

    match seq {
        ast::Seq::Loop(..) |
        ast::Seq::While(..) => {
            // Extract the conditional as an option.
            let (cond, mut body) = match seq {
                ast::Seq::Loop(ast::SeqBlock(body)) => (None, body),
                ast::Seq::While(cond, ast::SeqBlock(body)) => (Some(cond), body),
                _ => unreachable!(),
            };

            // Expand await statements. Split into yield blocks.
            body = expand_await(body);
            let mut spans = fsm_split_yield(body);
            assert!(spans.len() > 1, "loop statements require one yield");

            let first = spans.remove(0);
            let last = spans.pop().expect("Lacking last span.");

            let mut inner_cases = vec![];
            let mut last_id = id;
            for span in spans.into_iter().rev() {
                // Parse this span as its own content.
                let (case, span_cases) = fsm_span(global, span, vec![], Some(last_id));
                if let Some(case) = case {
                    last_id = case.states[0];
                    inner_cases.push(case);
                }
                inner_cases.extend(span_cases);
            }

            global.counter -= 1; // see below
            // Parse the first and last blocks.
            let (first_block, first_cases) = fsm_span(global, first, vec![], Some(last_id));

            let mut case = FsmCase {
                states: vec![id],
                body: vec![],
            };

            if let Some(mut cond) = cond {
                // Poor some sugar on this
                cond = ast::Expr::Arith(ast::Op::And, Box::new(fsm_match_list(ast::Op::Eq, &[444]).unwrap()), Box::new(cond));

                if let Some(FsmCase {
                    states: first_states,
                    body: first_body,
                }) = first_block {
                    let seq = ast::Seq::If(cond,
                        ast::SeqBlock(first_body),
                        Some(ast::SeqBlock(after)));
                    case.states.extend(&first_states[1..]);
                    case.body.push(seq);
                } else {
                    let seq = ast::Seq::If(invert_expr(cond),
                        ast::SeqBlock(after),
                        None);
                    case.body.push(seq);
                }
            } else {
                let first_case = first_block.expect("Lacking first case in loop.");
                case.states.extend(&first_case.states[1..]);
                case.body.extend(first_case.body);
            }

            println!("LOOP {:?}", case.body);

            let (mut last_block, last_cases) = fsm_span(global, last, case.body.clone(), None);
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

    println!("");
    let mut global = FsmGlobal {
        counter: 0
    };
    let (case, mut cases) = fsm_structure(&mut global, vec![], loop_seq);
    cases.insert(0, case);

    println!("cases {:?}", cases);

    // Match cases
    let mut output: Vec<(Vec<ast::Expr>, ast::SeqBlock)> = vec![];
    for case in cases {
        output.push((
            case.states.iter().sorted().iter().map(|x| ast::Expr::Num(**x)).collect(),
            ast::SeqBlock(case.body)));
    }

    (ast::Seq::Match(ast::Expr::Ref(ast::Ident("_FSM".to_string())),
        output), v.clone())
}
