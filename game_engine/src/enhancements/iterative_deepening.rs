use super::super::{
    function_types::{DepthSearchFunction, DepthSearchFunctionWithTable},
    traits::{ChildStates, ScoreOfState, StateHash, TerminalState},
};
use crossbeam::{channel::unbounded, thread};
use std::{
    cmp::{max, min},
    marker::{Send, Sync},
    time::{Duration, Instant},
};

use super::transposition_table::TranspositionTable;
pub fn iterative_deepening_t_tt<M, T>(
    state: &T,
    color: bool,
    max_time: Duration,
    table: &mut TranspositionTable<M>,
    search_function: fn(&T, isize, bool, &mut TranspositionTable<M>) -> isize,
) -> isize
where
    T: ScoreOfState + TerminalState + ChildStates<M> + StateHash + Sync,
    M: Send,
{
    let mut max_score = isize::MIN + 1;
    let mut min_score = isize::MAX;
    let mut depth = 1;
    let start = Instant::now();
    while start.elapsed() < max_time {
        let (scval, rcval) = unbounded();
        thread::scope(|s| {
            s.spawn(|_| {
                let val = search_function(state, depth, color, table);
                scval.send(val).unwrap();
                drop(scval);
            });
        })
        .unwrap();
        if max_time < start.elapsed() {
            break;
        }
        let score = match rcval.recv_timeout(max_time - start.elapsed()) {
            Ok(x) => x,
            Err(_) => break,
        };
        max_score = max(score, max_score);
        min_score = min(score, min_score);
        depth += 1;
    }
    match color {
        true => max_score,
        false => min_score,
    }
}
pub fn iterative_deepening_tt<M, T>(
    state: &T,
    max_depth: isize,
    color: bool,
    table: &mut TranspositionTable<M>,
    search_function: DepthSearchFunctionWithTable<M, T>,
) -> isize
where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + StateHash,
{
    let mut max_score = isize::MIN + 1;
    let mut min_score = isize::MAX;

    for depth in 1..max_depth {
        let score = search_function(state, depth, color, table);
        max_score = max(score, max_score);
        min_score = min(score, min_score);
    }
    match color {
        true => max_score,
        false => min_score,
    }
}

pub fn iterative_deepening<M, T>(
    state: &T,
    max_depth: isize,
    color: bool,
    search_function: DepthSearchFunction<T>,
) -> isize
where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    let mut max_score: isize = isize::MIN + 1;
    let mut min_score: isize = isize::MAX;

    for depth in 1..max_depth {
        let score = search_function(state, depth, color);
        max_score = max(score, max_score);
        min_score = min(score, min_score);
    }
    match color {
        true => max_score,
        false => min_score,
    }
}

pub fn iterative_deepening_t<M, T>(
    state: &T,
    max_time: Duration,
    color: bool,
    search_function: DepthSearchFunction<T>,
) -> isize
where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    let mut max_score: isize = isize::MIN + 1;
    let mut min_score: isize = isize::MAX;
    let mut depth: isize = 1;
    let start = Instant::now();
    while start.elapsed() < max_time {
        let score = search_function(state, depth, color);
        max_score = max(score, max_score);
        min_score = min(score, min_score);
        depth += 1;
    }
    match color {
        true => max_score,
        false => min_score,
    }
}
