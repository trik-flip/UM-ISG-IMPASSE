extern crate rand;
use std::cmp::{max, min};
use std::fmt::Display;

use super::{
    super::traits::{ChildStates, ScoreOfState, StateHash, TerminalState},
    transposition_state_type::TranspositionStateType,
    transposition_table::TranspositionTable,
};

pub fn replay_game<M: Copy + Display, T: ChildStates<M> + Copy + Display>(
    &state: &T,
    moves: &Vec<M>,
) {
    let mut next_state = state;
    for &next_move in moves {
        next_state = next_state + next_move;
        println!("{}", next_move);
        println!("{}", next_state);
    }
}

pub fn nega<M, T>(&state: &T, depth: isize, color: bool, mut alpha: isize, beta: isize) -> isize
where
    M: Copy + Ord,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    if 0 == depth || state.is_terminal() {
        return match color {
            true => state.score_of(),
            false => -state.score_of(),
        };
    }

    let moves = state.child_states(color);
    let mut score = isize::MIN + 1;
    for new_move in moves {
        let child = state + new_move;
        let value = -nega(&child, depth - 1, !color, -beta, -alpha);
        score = max(score, value);
        alpha = max(alpha, score);
        if alpha >= beta {
            return alpha;
        }
    }
    score
}

pub fn alpha_beta<M, T>(
    &state: &T,
    depth: isize,
    color: bool,
    mut alpha: isize,
    mut beta: isize,
) -> isize
where
    M: Copy + Ord,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    if 0 == depth || state.is_terminal() {
        return state.score_of();
    }
    let moves = state.child_states(color);

    let mut score: isize;
    if color {
        score = isize::MIN + 1;
        for new_move in moves {
            let child = state + new_move;
            let value = alpha_beta(&child, depth - 1, !color, alpha, beta);
            score = max(score, value);
            alpha = max(alpha, score);
            if score >= beta {
                break;
            }
        }
    } else {
        score = isize::MAX;
        for child in moves {
            let child_state = state + child;
            let child_value = alpha_beta(&child_state, depth - 1, !color, alpha, beta);
            score = min(score, child_value);
            beta = min(score, beta);
            if score <= alpha {
                break;
            }
        }
    }
    score
}

pub fn alpha_beta_bitwise<M, T>(
    &state: &T,
    depth: isize,
    color: usize,
    mut alpha: isize,
    mut beta: isize,
) -> isize
where
    M: Copy,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    if 0 == depth || state.is_terminal() {
        return state.score_of();
    }
    let vals = [false, true];
    let moves = state.child_states(vals[color]);

    let mut value = 0;
    let ab_funcs: [BitwisePart<T, M>; 2] = [ab_bitwise_min_part, ab_bitwise_max_part];
    ab_funcs[color](
        &mut value,
        &moves,
        state,
        depth,
        color ^ 1,
        &mut alpha,
        &mut beta,
    );
    value
}

fn ab_bitwise_min_part<T, M>(
    value: &mut isize,
    moves: &Vec<M>,
    state: T,
    depth: isize,
    color: usize,
    alpha: &mut isize,
    beta: &mut isize,
) where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
    M: Copy,
{
    *value = isize::MAX;
    for new_move in moves {
        let child_state = state + *new_move;
        let child_value = alpha_beta_bitwise(&child_state, depth - 1, color, *alpha, *beta);
        *value = min(*value, child_value);
        *beta = min(*value, *beta);
        if *value <= *alpha {
            break;
        }
    }
}
type BitwisePart<T, M> = fn(&mut isize, &Vec<M>, T, isize, usize, &mut isize, &mut isize);
fn ab_bitwise_max_part<T, M>(
    value: &mut isize,
    moves: &Vec<M>,
    state: T,
    depth: isize,
    color: usize,
    alpha: &mut isize,
    beta: &mut isize,
) where
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
    M: Copy,
{
    *value = isize::MIN + 1;
    for new_move in moves {
        let child_state = state + *new_move;
        let child_value = alpha_beta_bitwise(&child_state, depth - 1, color, *alpha, *beta);
        *value = max(*value, child_value);
        *alpha = max(*alpha, *value);
        if *value >= *beta {
            break;
        }
    }
}

// ! ====================== This is the one =====================
pub fn nega_with_table<M, T>(
    &state: &T,
    depth: isize,
    color: bool,
    table: &mut TranspositionTable<M>,
    mut alpha: isize,
    mut beta: isize,
) -> isize
where
    M: Default + Copy + Eq + Ord,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + StateHash,
{
    // ! ================= Save alpha ===================
    let original_alpha = alpha;

    // ! =================== Check TT ==================
    let (stored_value, stored_depth, state_type, stored_best_move) = table.get(state.hash(color));
    if state_type != TranspositionStateType::Unknown && stored_depth >= depth {
        match state_type {
            TranspositionStateType::Exact => return stored_value,
            TranspositionStateType::LowerBound => alpha = max(alpha, stored_value),
            TranspositionStateType::UpperBound => beta = min(beta, stored_value),
            _ => panic!(),
        }
        if alpha >= beta {
            return stored_value;
        }
    }

    if 0 == depth || state.is_terminal() {
        return match color {
            true => state.score_of(),
            false => -state.score_of(),
        };
    }

    let children = state.child_states(color);
    let mut ordered_children = children.clone();

    // ! ===================== Check TT Move First ====================
    if state_type != TranspositionStateType::Unknown {
        for (index, child) in children.into_iter().enumerate() {
            if child == stored_best_move {
                let temp = ordered_children.remove(index);
                ordered_children.insert(0, temp);
                break;
            }
        }
    }

    let mut value: isize;
    let mut best_move = M::default();
    value = isize::MIN + 1;
    // !========================== Nega Alpha-Beta ===========================
    for child in ordered_children {
        let child_state = state + child;
        let child_value = -nega_with_table(&child_state, depth - 1, !color, table, -beta, -alpha);
        if value < child_value {
            best_move = child;
        }
        value = max(value, child_value);
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }
    // ! ==================== Store in TT ===================
    let flag: TranspositionStateType;
    if value <= original_alpha {
        flag = TranspositionStateType::UpperBound;
    } else if value >= beta {
        flag = TranspositionStateType::LowerBound;
    } else {
        flag = TranspositionStateType::Exact;
    }
    table.add(state.hash(color), (value, depth, flag, best_move));

    value
}

pub fn alpha_beta_with_table<M, T>(
    &state: &T,
    depth: isize,
    color: bool,
    table: &mut TranspositionTable<M>,
    mut alpha: isize,
    mut beta: isize,
) -> isize
where
    M: Default + Copy + Eq + Ord,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy + StateHash,
{
    let original_alpha = alpha;

    // Check TT
    let (stored_value, stored_depth, state_type, stored_best_move) = table.get(state.hash(color));
    if state_type != TranspositionStateType::Unknown && stored_depth >= depth {
        match state_type {
            TranspositionStateType::Exact => return stored_value,
            TranspositionStateType::LowerBound => alpha = max(alpha, stored_value),
            TranspositionStateType::UpperBound => beta = min(beta, stored_value),
            _ => panic!(),
        }
        if alpha >= beta {
            return stored_value;
        }
    }

    if 0 == depth || state.is_terminal() {
        return state.score_of();
    }

    let children = state.child_states(color);
    let mut ordered_children = children.clone();

    if state_type != TranspositionStateType::Unknown {
        for (index, child) in children.into_iter().enumerate() {
            if child == stored_best_move {
                let temp = ordered_children.remove(index);
                ordered_children.insert(0, temp);
                break;
            }
        }
    }

    let mut value: isize;
    let mut best_move = M::default();
    if color {
        value = isize::MIN + 1;
        for child in ordered_children {
            let child_state = state + child;
            let child_value =
                alpha_beta_with_table(&child_state, depth - 1, !color, table, alpha, beta);
            if value < child_value {
                best_move = child;
            }
            value = max(value, child_value);
            alpha = max(alpha, value);
            if value >= beta {
                break;
            }
        }
    } else {
        value = isize::MAX;
        for child in ordered_children {
            let child_state = state + child;
            let child_value =
                alpha_beta_with_table(&child_state, depth - 1, !color, table, alpha, beta);
            if value > child_value {
                best_move = child;
            }
            value = min(value, child_value);
            beta = min(beta, value);
            if value <= alpha {
                break;
            }
        }
    }
    let flag: TranspositionStateType;
    if value <= original_alpha {
        flag = TranspositionStateType::UpperBound;
    } else if value >= beta {
        flag = TranspositionStateType::LowerBound;
    } else {
        flag = TranspositionStateType::Exact;
    }
    table.add(state.hash(color), (value, depth, flag, best_move));

    value
}
pub fn nega_scout<M, T>(
    state: &T,
    depth: isize,
    color: bool,
    mut alpha: isize,
    beta: isize,
) -> isize
where
    M: Copy + Ord,
    T: ScoreOfState + TerminalState + ChildStates<M> + Copy,
{
    if depth == 0 || state.is_terminal() {
        return match color {
            true => state.score_of(),
            false => -state.score_of(),
        };
    }

    let mut score = isize::MIN + 1;
    let mut n = beta;
    let moves = state.child_states(color);

    for new_move in moves {
        let child = *state + new_move;
        let value = -nega_scout(&child, depth - 1, !color, -n, -alpha);
        if value > score {
            if n == beta || depth <= 2 {
                score = value;
            } else {
                score = -nega_scout(state, depth - 1, !color, -beta, -value);
            }
        }
        alpha = max(alpha, score);
        if alpha >= beta {
            return alpha;
        }
        n = alpha + 1;
    }
    score
}
