use std::fmt::Display;

use super::{move_type::MoveType, position::Position};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Move {
    pub positions: [Position; 3],
    pub move_type: MoveType,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.move_type).unwrap();
        for pos in self.positions {
            if pos.old_sign != 3 && pos.new_sign != 3 && pos.new_sign != pos.old_sign {
                writeln!(f, "{}", pos).unwrap();
            }
        }
        Ok(())
    }
}
impl Default for Move {
    fn default() -> Self {
        Move {
            move_type: MoveType::Invalid,
            positions: Default::default(),
        }
    }
}
impl Move {
    pub fn is_valid(&self) -> bool {
        self.move_type != MoveType::Invalid
    }
    pub fn to_bear_off(&mut self) {
        self.move_type = self.move_type + MoveType::BearOff;
        if self.positions[0].new_sign.abs() == 2 {
            self.positions[0].new_sign /= 2
        } else if self.positions[1].new_sign.abs() == 2 {
            self.positions[1].new_sign /= 2
        } else {
            panic!();
        }
    }
}
