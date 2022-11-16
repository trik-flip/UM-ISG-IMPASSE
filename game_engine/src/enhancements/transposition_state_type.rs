#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TranspositionStateType {
    Exact = 0,
    LowerBound = 1,
    UpperBound = 2,
    Unknown = 3,
}
