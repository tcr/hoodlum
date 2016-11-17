use VerilogState;
use ast;

use itertools::Itertools;
use std::mem;
use std::collections::HashMap;
use CountWalker;
use Walkable;

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

    let mut list = list.to_vec();
    list.reverse();

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

#[derive(Clone, Debug)]
enum Transition {
    Yield(i32),
    Precede(Vec<i32>),
}

fn normalize<T: ::std::hash::Hash + Eq + Clone + Ord>(input: &[T]) -> Vec<T> {
    input.to_vec().into_iter().unique().sorted()
}

fn fsm_span(global: &mut FsmGlobal, mut body: Vec<ast::Seq>, before: Option<i32>, after: FsmCase, transition: Transition) -> (Option<FsmCase>, Vec<FsmCase>) {
    let mut other_cases = vec![];
    let mut case = FsmCase {
        states: vec![],
        body: vec![]
    };

    // Terminate early.
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
                println!("--------> {:?} {:?}", transition, before);
                let following_transition = match transition {
                    Transition::Precede(ref next) => {
                        let mut n = next.clone();
                        n.push(global.counter);
                        Transition::Precede(n)
                    }
                    Transition::Yield(ref next) => {
                        transition.clone()
                    }
                };
                let gc = global.counter;
                let (mut case, mut other_cases) = fsm_span(global, following, Some(gc), after, following_transition);
                let mut case = case.expect("missing a case");

                // Parse loop with our merged "after" and "following" blocks.
                println!("STRUCT {:?} {:?}", before, case);
                let (structure, other) = fsm_structure(global, before, case.clone(), seq);
                case.states.extend(&structure.states);
                mem::replace(&mut case.body, structure.body);
                other_cases.extend(other);

                // Parse the remaining content in "body" as its own span.
                let next_transition = if case.states.len() > 0 {
                    if let Some(before) = before {
                        Transition::Precede(vec![before])
                    } else {
                        Transition::Precede(vec![case.states[0]])
                    }
                } else {
                    Transition::Yield(999)
                };
                println!("1 {:?} vs {:?}", next_transition, case.states);
                if let (Some(preceding), other) = fsm_span(global, body, before, case.clone(), next_transition) {
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
                    global.counter += 1;

                    inner.push(ast::Seq::FsmTransition(target as u32));
                    case.body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &[id as u32]).unwrap(),
                        ast::SeqBlock(inner),
                        None));
                    case.states.push(id);

                }
                Transition::Precede(targets) => {
                    let mut states: Vec<u32> = targets.iter().map(|x| *x as _).collect();
                    states = normalize(&states);

                    let n = normalize(&after.states);
                    // TODO this is weird logic to make rewrite_await8 work
                    println!("~~~~~~~~> {:?} {:?}", n, targets);
                    if n.len() > 1 && targets.len() > 1 {
                        inner.push(ast::Seq::FsmTransition(*n.last().unwrap() as u32));
                    }

                    case.body.push(ast::Seq::If(fsm_match_list(ast::Op::Eq, &states).unwrap(),
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
                global.counter += 1;

                case.body.push(ast::Seq::FsmTransition(target as u32));
                case.states.push(id);
            }
            Transition::Precede(..) => {
                // ignore
            }
        }
    }

    (Some(case), other_cases)
}

/// Converts a sequence Loop, While, or If statement into a set of cases.
fn fsm_structure(global: &mut FsmGlobal, before: Option<i32>, after: FsmCase, seq: ast::Seq) -> (FsmCase, Vec<FsmCase>) {
    let mut id = global.counter;
    global.counter += 1;

    match seq {
        ast::Seq::Loop(..) |
        ast::Seq::While(..) |
        ast::Seq::If(..) => {
            // Extract the conditional as an option.
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
                let glob_id = global.counter;
                let (case, span_cases) = fsm_span(global, span, Some(glob_id), FsmCase {
                    body: vec![],
                    states: vec![]
                }, Transition::Yield(last_id));
                if let Some(case) = case {
                    last_id = case.states[0];
                    inner_cases.push(case);
                }
                inner_cases.extend(span_cases);
            }

            println!("WHAT {:?}", global.counter);

            // Decrease global counter?
            //TODO explain this
            global.counter -= 1;
            // Parse the first block.
            println!("FIRST {:?} {:?}", id, before);
            let (first_block, first_cases) = fsm_span(global, first, Some(id), FsmCase {
                body: vec![],
                states: vec![]
            }, Transition::Yield(last_id));

            let mut case = FsmCase {
                states: vec![id],
                body: vec![],
            };

            // Generate a state whitelist.
            let mut state_whitelist = vec![];
            if !is_if {
                state_whitelist.push(id as u32);
            }
            if let Some(before) = before {
                state_whitelist.push(before as u32);
            }
            state_whitelist = normalize(&state_whitelist);
            println!("-----> LOOP WHITELIST {:?} {:?} {:?}", state_whitelist, id, before);

            // Generate initial "first" case. Output depends on if we have a
            // condition (if/while) or not (loop)
            if let Some(mut cond) = cond {
                // If our first case has content, generate an if {...} else {...}
                // loop. Otherwise, invert the conditional, just generate an
                // if !cond {...}
                if let Some(FsmCase {
                    states: first_states,
                    body: first_body,
                }) = first_block {
                    // Change our condition to also check our FSM states.
                    cond = ast::Expr::Arith(ast::Op::And, Box::new(fsm_match_list(ast::Op::Eq, &state_whitelist).unwrap()), Box::new(cond));

                    let seq = ast::Seq::If(cond,
                        ast::SeqBlock(first_body),
                        Some(ast::SeqBlock(after.body)));
                    case.states.extend(&first_states[1..]);
                    case.body.push(seq);
                } else {
                    // Change our condition to also check our FSM states.
                    cond = ast::Expr::Arith(ast::Op::And, Box::new(fsm_match_list(ast::Op::Eq, &state_whitelist).unwrap()), Box::new(cond));

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
                if let Some(before) = before {
                    if before != id && first_case.states.len() > 1 {
                        let seq = ast::Seq::If(fsm_match_list(ast::Op::Eq, &[before as u32]).unwrap(),
                            ast::SeqBlock(vec![
                                ast::Seq::FsmTransition(id as u32),
                            ]),
                            None);
                        case.body.insert(0, seq);
                    }
                }
            }

            if last.is_none() {
                assert!(has_loop);
                let mut other_cases = vec![];
                other_cases.extend(first_cases);
                other_cases.extend(inner_cases);
                return (case, other_cases);
            }

            // Generate "last" block with all previous generated content as its
            // "after" content.
            let last = last.expect("missing last span");
            let (mut last_block, last_cases) = fsm_span(global, last, Some(id), case.clone(), Transition::Precede(vec![id]));
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
    let (case, mut cases) = fsm_structure(&mut global, Some(0), FsmCase {
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
