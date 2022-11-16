use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use super::enhancements::transposition_table::TranspositionTable;
/// A basic or ID alpha beta search
pub type DepthSearchFunction<T> = fn(&T, isize, bool) -> isize;
/// A Timed ID alpha beta search
pub type TimedSearchFunction<T> = fn(&T, Duration, bool) -> isize;
/// A Timed ID with min depth alpha beta search
pub type TimedDepthSearchFunction<T> = fn(&T, isize, Duration, bool) -> isize;
/// A basic or ID alpha beta search with TT
pub type DepthSearchFunctionWithTable<M, T> =
    fn(&T, isize, bool, &mut TranspositionTable<M>) -> isize;
pub type DepthSearchFunctionWithTableMutex<M, T> =
    fn(&T, isize, bool, Arc<Mutex<&mut TranspositionTable<M>>>) -> isize;
pub type TimedSearchFunctionWithTable<M, T> =
    fn(&T, bool, Duration, &mut TranspositionTable<M>) -> isize;
