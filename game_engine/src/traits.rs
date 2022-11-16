use std::ops::{Add, Sub};

pub trait ChildStates<T>
where
    Self: Add<T, Output = Self> + Sub<T, Output = Self>,
{
    fn child_states(&self, color: bool) -> Vec<T>;
}
pub trait ScoreOfState {
    fn score_of(&self) -> isize;
}
pub trait TerminalState {
    fn is_terminal(&self) -> bool;
}
pub trait StateHash {
    fn hash(&self, color: bool) -> isize;
}
