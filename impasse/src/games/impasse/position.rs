use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub old_sign: isize,
    pub new_sign: isize,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            x: 0,
            y: 0,
            old_sign: 3,
            new_sign: 3,
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} - from {} to {}",
            to_alphabet(self.y),
            8 - self.x,
            self.old_sign,
            self.new_sign
        )
    }
}

const fn to_alphabet(y: usize) -> char {
    (y + 65) as u8 as char
}
