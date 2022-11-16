use std::collections::HashMap;

use super::transposition_state_type::TranspositionStateType;

/// A hashmap used to store known states
unsafe impl<M> Send for TranspositionTable<M> where M: Send {}
unsafe impl<M> Sync for TranspositionTable<M> where M: Sync {}
pub struct TranspositionTable<M> {
    map: HashMap<isize, (isize, isize, TranspositionStateType, M)>,
}

impl<M> Default for TranspositionTable<M> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
impl<M> TranspositionTable<M>
where
    M: Default + Copy,
{
    pub const RETRY_VALUE: isize = 0;
    pub fn cap(&self) -> usize {
        self.map.capacity()
    }
    pub fn size(&self) -> usize {
        self.map.len()
    }

    pub(crate) fn add(&mut self, key: isize, value: (isize, isize, TranspositionStateType, M)) {
        self.map.insert(key, value);
    }
    pub(crate) fn get(&self, key: isize) -> (isize, isize, TranspositionStateType, M) {
        match self.map.get(&key) {
            Some(&x) => x,
            None => (0, 0, TranspositionStateType::Unknown, M::default()),
        }
    }
}
