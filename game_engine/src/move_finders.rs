use std::{
    fmt::Display,
    io::stdin,
    time::{Duration, Instant},
};

use crossbeam::{channel::unbounded, thread};
use rand::{rngs::StdRng, seq::SliceRandom};

use super::{
    enhancements::transposition_table::TranspositionTable,
    function_types::{
        DepthSearchFunction, DepthSearchFunctionWithTable, TimedSearchFunction,
        TimedSearchFunctionWithTable,
    },
    traits::{ChildStates, ScoreOfState, TerminalState},
};
pub fn random_agent<T: ChildStates<M>, M: Copy>(
    current_move: T,
    color: bool,
    seed: &mut StdRng,
) -> M {
    *current_move.child_states(color).choose(seed).unwrap()
}
pub fn human_agent<T: ChildStates<M>, M: Display + Copy>(current_move: &T, color: bool) -> M {
    let children = current_move.child_states(color);
    let mut index = 1;
    for child in children.iter() {
        println!("{}index:{}\n", child, index);
        index += 1;
    }
    let mut string = String::new();
    match stdin().read_line(&mut string) {
        Ok(_) => {
            let numbers = string
                .strip_suffix("\r\n")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            children[numbers - 1]
        }
        _ => panic!(),
    }
}
pub fn find_best_move_tt<
    M: Copy,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + PartialEq,
>(
    state: &T,
    depth: isize,
    color: bool,
    table: &mut TranspositionTable<M>,
    search_function: DepthSearchFunctionWithTable<M, T>,
) -> M {
    let mut best_score = match color {
        true => isize::MIN + 1,
        false => isize::MAX,
    };
    let moves = state.child_states(color);
    if moves.len() == 1 {
        return moves[0];
    }
    let mut best_move = moves[0];

    for next_move in moves {
        let child_state = *state + next_move;
        let score = search_function(&child_state, depth, !color, table);
        if color && (score > best_score) || !color && (score < best_score) {
            best_move = next_move;
            best_score = score;
        }
    }
    best_move
}

pub fn find_best_move_t_tt<T, M>(
    state: &T,
    max_time: Duration,
    color: bool,
    table: &mut TranspositionTable<M>,
    search_function: TimedSearchFunctionWithTable<M, T>,
) -> M
where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + PartialEq,
    M: Copy,
{
    let mut best_score = match color {
        true => isize::MIN + 1,
        false => isize::MAX,
    };
    let moves = state.child_states(color);
    if moves.len() == 1 {
        return moves[0];
    }
    let mut best_move = moves[0];

    let total_time = max_time / moves.len() as u32;
    for next_move in moves {
        let child = *state + next_move;
        let score = search_function(&child, !color, total_time, table);
        if color && (score > best_score) || !color && (score < best_score) {
            best_move = next_move;
            best_score = score;
        }
    }
    best_move
}
pub fn find_best_move_t_tt_id<T, M>(
    state: &T,
    max_time: Duration,
    color: bool,
    table: &mut TranspositionTable<M>,
    search_function: DepthSearchFunctionWithTable<M, T>,
) -> M
where
    T: ScoreOfState + TerminalState + ChildStates<M> + PartialEq + Sync + Copy,
    M: Send + Copy,
{
    let mut best_score = match color {
        true => isize::MIN + 1,
        false => isize::MAX,
    };
    let moves = state.child_states(color);

    if moves.len() == 1 {
        return moves[0];
    }

    let mut best_move = moves[0];
    let mut depth = 1;
    let start = Instant::now();
    while start.elapsed() < max_time {
        for next_move in &moves {
            let child = *state + *next_move;
            let (scval, rcval) = unbounded();

            // !========================= Start a new thread ==================================
            thread::scope(|s| {
                s.spawn(|_| {
                    let val = search_function(&child, depth, !color, table);
                    scval.send(val).unwrap();
                    drop(scval);
                });
            })
            .unwrap();
            if max_time < start.elapsed() {
                break;
            }
            // !============================ Get result if there is no timeout =================
            let score = match rcval.recv_timeout(max_time - start.elapsed()) {
                Ok(x) => x,
                Err(_) => break,
            };

            if color && (score > best_score) || !color && (score < best_score) {
                best_move = *next_move;
                best_score = score;
            }
        }
        depth += 1;
    }
    println!("Thinking {}ply deep", depth - 1);
    best_move
}
pub fn find_best_move_t<T, M>(
    state: &T,
    max_time: Duration,
    color: bool,
    search_function: TimedSearchFunction<T>,
) -> M
where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + PartialEq,
    M: Copy,
{
    let moves = state.child_states(color);
    if moves.len() == 1 {
        return moves[0];
    }
    let mut best_score = match color {
        true => isize::MIN + 1,
        false => isize::MAX,
    };
    let mut best_move = moves[0];

    let total_time = max_time / moves.len() as u32;
    for next_move in moves {
        let score = search_function(&(*state + next_move), total_time, !color);
        if color && (score > best_score) || !color && (score < best_score) {
            best_move = next_move;
            best_score = score;
        }
    }
    best_move
}
pub fn find_best_move<M, T>(
    state: &T,
    depth: isize,
    color: bool,
    search_function: DepthSearchFunction<T>,
) -> M
where
    M: Copy,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + PartialEq,
{
    let moves = state.child_states(color);
    if moves.len() == 1 {
        return moves[0];
    }

    let mut best_score = match color {
        true => isize::MIN + 1,
        false => isize::MAX,
    };
    let mut best_move = moves[0];
    for next_move in moves {
        let child_state = *state + next_move;
        let score = search_function(&child_state, depth, !color);
        if color && (score > best_score) || !color && (score < best_score) {
            best_move = next_move;
            best_score = score;
        }
    }
    best_move
}
