use std::{fmt::Display, ops};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum MoveType {
    Invalid,
    Normal,
    Transpose,
    Crown,
    TransposeCrown,
    TransposeBearOff,
    BearOff,
    Impasse,
    BearOffCrown,
    ImpasseCrown,
}
impl Default for MoveType {
    fn default() -> Self {
        Self::Invalid
    }
}
impl MoveType {
    fn string_map(self) -> String {
        match self {
            MoveType::Invalid => String::from("Invalid"),
            MoveType::BearOff => String::from("BearOff"),
            MoveType::BearOffCrown => String::from("BearOffCrown"),
            MoveType::Crown => String::from("Crown"),
            MoveType::Impasse => String::from("Impasse"),
            MoveType::ImpasseCrown => String::from("ImpasseCrown"),
            MoveType::Normal => String::from("Normal"),
            MoveType::Transpose => String::from("Transpose"),
            MoveType::TransposeCrown => String::from("TransposeCrown"),
            MoveType::TransposeBearOff => String::from("TransposeBearOff"),
        }
    }
}

impl Display for MoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_map())
    }
}
impl ops::Add<MoveType> for MoveType {
    type Output = MoveType;

    fn add(self, rhs: MoveType) -> Self::Output {
        if self == Self::Normal {
            return rhs;
        } else if self == Self::BearOff {
            return match rhs {
                Self::Crown => Self::BearOffCrown,
                _ => panic!("Isn't possible to add {} with {}", self, rhs),
            };
        } else if self == Self::Transpose {
            return match rhs {
                Self::Crown => Self::TransposeCrown,
                Self::BearOff => Self::TransposeBearOff,
                _ => panic!("Isn't possible to add {} with {}", self, rhs),
            };
        } else if self == Self::Impasse {
            return match rhs {
                Self::Crown => Self::ImpasseCrown,
                _ => panic!("Isn't possible to add {} with {}", self, rhs),
            };
        }
        panic!("Isn't possible to add {} with {}", self, rhs);
    }
}
