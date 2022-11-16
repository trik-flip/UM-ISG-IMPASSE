use std::{fmt::Display, isize, ops};

use rand::{rngs::StdRng, Rng, SeedableRng};

use game_engine::traits::{ChildStates, ScoreOfState, StateHash, TerminalState};
type TableSize = u64;
type HashField = [[[TableSize; 4]; 8]; 8];

use super::actions::Move;
use super::move_type::MoveType;
use super::position::Position;
use super::GameField;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Impasse<'game> {
    pub game_field: GameField,
    pub hash_field: &'game HashField,
}

impl ops::Sub<Move> for Impasse<'_> {
    type Output = Self;
    fn sub(self, new_move: Move) -> Self::Output {
        let mut board = self;
        for pos in new_move.positions {
            if pos.new_sign != 3 && pos.old_sign != 3 {
                board.game_field[pos.x][pos.y] = pos.old_sign;
            }
        }
        board
    }
}

impl ops::Add<Move> for Impasse<'_> {
    type Output = Self;
    fn add(self, new_move: Move) -> Self::Output {
        let mut board = self;
        for pos in new_move.positions {
            if pos.new_sign != 3 && pos.old_sign != 3 {
                board.game_field[pos.x][pos.y] = pos.new_sign;
            }
        }
        board
    }
}

impl ScoreOfState for Impasse<'_> {
    fn score_of(&self) -> isize {
        let mut pos_pieces = 0;
        let mut neg_pieces = 0;
        let mut score = 10000;
        for i in 0..8 {
            for j in 0..8 {
                score = match self.game_field[i][j] {
                    0 => score,
                    1 => score - (7 + i),
                    2 => score - (21 - i),
                    -1 => score + (14 - i),
                    -2 => score + (14 + i),
                    _ => panic!("{}", self.game_field[i][j]),
                };
                match self.game_field[i][j] {
                    0 => (),
                    -1 => neg_pieces += 1,
                    -2 => neg_pieces += 2,
                    1 => pos_pieces += 1,
                    2 => pos_pieces += 2,
                    _ => panic!(),
                }
            }
        }
        if pos_pieces == 0 {
            return isize::MAX;
        } else if neg_pieces == 0 {
            return isize::MIN + 1;
        }
        score as isize + (neg_pieces - pos_pieces) * 10
    }
}

impl ChildStates<Move> for Impasse<'_> {
    fn child_states(&self, color: bool) -> Vec<Move> {
        let mut possible_moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                self.move_normal(&mut possible_moves, color, i, j);
            }
        }
        if !possible_moves.is_empty() {
            return possible_moves;
        }

        let sign = match color {
            true => 1,
            false => -1,
        };

        for i in 0..8 {
            for j in 0..8 {
                self.move_impasse(&mut possible_moves, color, i, j, sign);
            }
        }
        possible_moves
    }
}

impl Display for Impasse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            write!(f, "{} ", 8 - i,).unwrap();
            for j in 0..8 {
                if (i + j) % 2 != 0 {
                    write!(f, "{} ", get_char(self.game_field[i][j])).unwrap();
                } else {
                    write!(f, "- ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f).unwrap();
        writeln!(f, "  A B C D E F G H").unwrap();
        Ok(())
    }
}

// Check moves
impl<'game> Impasse<'game> {
    pub const fn new(hash_field: &'game HashField) -> Self {
        Impasse {
            game_field: gen_default_field(),
            hash_field,
        }
    }
    const fn can_bear_off(&self, x: usize, y: usize) -> bool {
        self.game_field[x][y] == 2 && x == 7 || self.game_field[x][y] == -2 && x == 0
    }

    fn can_crown_self(&self, x: usize, y: usize) -> bool {
        if self.game_field[x][y].abs() != 1 || !(x == 0 || x == 7) {
            return false;
        }
        for i in 0..8 {
            for j in 0..8 {
                if self.is_possible_pawn(x, y, i, j) {
                    return true;
                }
            }
        }
        false
    }

    const fn can_transpose(&self, x: usize, y: usize, nex: usize, ney: usize) -> bool {
        x.abs_diff(nex) == 1
            && y.abs_diff(ney) == 1
            && self.game_field[x][y] * 2 == self.game_field[nex][ney]
    }

    fn can_crown_other(&self, color: bool) -> bool {
        for i in 0..8 {
            if color && self.game_field[0][i] == 1 || !color && self.game_field[7][i] == -1 {
                return true;
            }
        }
        false
    }
}

// Gen moves
impl Impasse<'_> {
    pub fn gen_hash_field(seed: u64) -> HashField {
        let mut seed = StdRng::seed_from_u64(seed);
        let field: HashField = seed.gen();
        // let field: HashField = rand::thread_rng().gen();
        field
    }

    fn gen_move_crown(
        &self,
        base: &Move,
        pawn_x: usize,
        pawn_y: usize,
        crown_x: usize,
        crown_y: usize,
    ) -> Move {
        let mut positions = base.positions;
        let mut set_pawn = false;
        let mut set_crown = false;
        let mut free_position = 0;
        let mut indexer_set = false;
        for (index, pos) in positions.iter_mut().enumerate() {
            if pos.x == pawn_x && pos.y == pawn_y {
                pos.new_sign = 0;
                set_pawn = true;
            } else if pos.x == crown_x && pos.y == crown_y {
                pos.new_sign *= 2;
                set_crown = true;
            }

            if *pos == Position::default() && !indexer_set {
                indexer_set = true;
                free_position = index;
            }
        }
        if !set_pawn {
            positions[free_position] = Position {
                x: pawn_x,
                y: pawn_y,
                old_sign: self.game_field[pawn_x][pawn_y],
                new_sign: 0,
            };
            free_position += 1;
        }
        if !set_crown {
            positions[free_position] = Position {
                x: crown_x,
                y: crown_y,
                old_sign: self.game_field[crown_x][crown_y],
                new_sign: self.game_field[crown_x][crown_y] * 2,
            };
        }

        Move {
            move_type: base.move_type + MoveType::Crown,
            positions,
        }
    }

    fn gen_move_impasse(&self, x: usize, y: usize) -> Move {
        let new_sign = match self.game_field[x][y].abs() {
            2 => self.game_field[x][y] / 2,
            1 => 0,
            _ => panic!("This isn't possible"),
        };
        Move {
            move_type: MoveType::Impasse,
            positions: [
                Position {
                    x,
                    y,
                    old_sign: self.game_field[x][y],
                    new_sign,
                },
                Position::default(),
                Position::default(),
            ],
        }
    }

    fn gen_move_normal(&self, x: usize, y: usize, new_x: usize, new_y: usize) -> Move {
        Move {
            move_type: MoveType::Normal,
            positions: [
                Position {
                    x,
                    y,
                    old_sign: self.game_field[x][y],
                    new_sign: 0,
                },
                Position {
                    x: new_x,
                    y: new_y,
                    old_sign: self.game_field[new_x][new_y],
                    new_sign: self.game_field[x][y],
                },
                Position::default(),
            ],
        }
    }

    fn gen_move_transpose(&self, x: usize, y: usize, nex: usize, ney: usize) -> Move {
        Move {
            move_type: MoveType::Transpose,
            positions: [
                Position {
                    x,
                    y,
                    old_sign: self.game_field[x][y],
                    new_sign: self.game_field[nex][ney],
                },
                Position {
                    x: nex,
                    y: ney,
                    old_sign: self.game_field[nex][ney],
                    new_sign: self.game_field[x][y],
                },
                Position::default(),
            ],
        }
    }
}
// Other functions
impl Impasse<'_> {
    fn in_final_row(&self, x: usize, y: usize) -> bool {
        x == 0 && self.game_field[x][y] == 1 || x == 7 && self.game_field[x][y] == -1
    }

    const fn is_free(&self, x: usize, y: usize) -> bool {
        self.game_field[x][y] == 0
    }

    fn move_impasse(&self, moves: &mut Vec<Move>, color: bool, x: usize, y: usize, sign: isize) {
        if self.game_field[x][y] * sign > 0 {
            let mut new_move = self.gen_move_impasse(x, y);

            let mut new_field = *self + new_move;
            // BearOff
            if new_field.can_bear_off(x, y) {
                new_field = new_field - new_move;
                new_move.to_bear_off();
                new_field = new_field + new_move;
            }

            new_field.do_crown(x, y, &new_move, moves, color);
        }
    }

    fn move_normal(&self, moves: &mut Vec<Move>, color: bool, x: usize, y: usize) {
        if !self.piece_is_valid(x, y, color) {
            return;
        }

        let mut front = color;

        if self.game_field[x][y].abs() == 2 {
            front = !front;
        }

        let mut left = true;
        let mut stop = false;

        let mut blocked = false;
        while !stop && (!blocked || left) {
            blocked = false;
            let mut counter: usize = 1;

            while !blocked
                && ((front && x >= counter || !front && x + counter < 8)
                    && (left && y >= counter || !left && y + counter < 8))
            {
                let (new_x, new_y) = calc_new_x_y(x, y, counter, front, left);
                let mut new_move: Move;
                blocked = true;

                if self.can_transpose(x, y, new_x, new_y) {
                    new_move = self.gen_move_transpose(x, y, new_x, new_y);
                } else if self.is_free(new_x, new_y) {
                    new_move = self.gen_move_normal(x, y, new_x, new_y);
                    blocked = false;
                } else {
                    new_move = Move::default();
                }

                if new_move.is_valid() {
                    let mut new_field = *self + new_move;
                    // BearOff
                    if new_field.can_bear_off(x, y) || new_field.can_bear_off(new_x, new_y) {
                        new_field = new_field - new_move;
                        new_move.to_bear_off();
                        new_field = new_field + new_move;
                    }

                    new_field.do_crown(new_x, new_y, &new_move, moves, color);
                }
                counter += 1;
            }
            if left {
                left = false;
                blocked = false;
            } else {
                stop = true;
            }
        }
    }

    fn piece_is_valid(&self, x: usize, y: usize, color: bool) -> bool {
        !(self.game_field[x][y] == 0
            || (color && self.game_field[x][y] < 0 || !color && self.game_field[x][y] > 0))
    }

    fn possible_crown_pawns(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut pawns = Vec::new();
        for x_other in 0..8 {
            for y_other in 0..8 {
                if self.is_possible_pawn(x, y, x_other, y_other) {
                    pawns.push((x_other, y_other));
                }
            }
        }
        pawns
    }
    /// Check if the same piece but not itself
    fn is_possible_pawn(&self, x: usize, y: usize, x_other: usize, y_other: usize) -> bool {
        (x != x_other || y != y_other) && self.game_field[x][y] == self.game_field[x_other][y_other]
    }
    fn waiting_crowns(&self, color: bool) -> Vec<usize> {
        let mut crowns = Vec::new();
        for i in 0..8 {
            if self.is_crown(color, i) {
                crowns.push(i);
            }
        }
        crowns
    }

    fn is_crown(&self, color: bool, i: usize) -> bool {
        color && self.game_field[0][i] == 1 || !color && self.game_field[7][i] == -1
    }

    fn do_crown(&self, x: usize, y: usize, new_move: &Move, moves: &mut Vec<Move>, color: bool) {
        if self.game_field[x][y].abs() != 1 {
            moves.push(*new_move);
            return;
        }

        let in_final_row = self.in_final_row(x, y);
        // Crown [Self]
        if in_final_row && self.can_crown_self(x, y) {
            for (pawn_x, pawn_y) in self.possible_crown_pawns(x, y) {
                let crown_move = self.gen_move_crown(new_move, pawn_x, pawn_y, x, y);
                moves.push(crown_move)
            }
        }
        // Crown [other]
        else if !in_final_row && self.can_crown_other(color) {
            let crown_x = match color {
                true => 0,
                false => 7,
            };
            for crown_y in self.waiting_crowns(color) {
                let crown_move = self.gen_move_crown(new_move, x, y, crown_x, crown_y);
                moves.push(crown_move)
            }
        } else {
            moves.push(*new_move);
        }
    }
}

impl TerminalState for Impasse<'_> {
    fn is_terminal(&self) -> bool {
        let mut pos_players: bool = false;
        let mut neg_players: bool = false;

        for i in 0..8 {
            for j in 0..8 {
                if self.game_field[i][j] > 0 {
                    pos_players = true;
                }
                if self.game_field[i][j] < 0 {
                    neg_players = true;
                }
            }
        }
        !pos_players || !neg_players
    }
}

impl StateHash for Impasse<'_> {
    fn hash(&self, color: bool) -> isize {
        let mut hash_val = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self.game_field[i][j] != 0 {
                    hash_val ^= self.hash_field[i][j][get_hash(self.game_field[i][j])];
                }
            }
        }
        hash_val ^= color as TableSize;
        hash_val as isize
    }
}

const fn get_hash(number: isize) -> usize {
    match number {
        -2 => 0,
        -1 => 1,
        1 => 2,
        2 => 3,
        _ => panic!(),
    }
}

const fn calc_new_x_y(
    x: usize,
    y: usize,
    counter: usize,
    front: bool,
    left: bool,
) -> (usize, usize) {
    let nex = match front {
        true => x - counter,
        false => x + counter,
    };
    let ney = match left {
        true => y - counter,
        false => y + counter,
    };
    (nex, ney)
}

const fn get_char(number: isize) -> char {
    match number {
        -2 => 'X',
        -1 => 'x',
        1 => 'o',
        2 => 'O',
        _ => '_',
    }
}
const fn gen_default_field() -> GameField {
    let mut game_field = [[0; 8]; 8];
    game_field[0][1] = 2;
    game_field[0][3] = -1;
    game_field[0][5] = 2;
    game_field[0][7] = -1;

    game_field[1][0] = -1;
    game_field[1][2] = 2;
    game_field[1][4] = -1;
    game_field[1][6] = 2;

    game_field[6][1] = -2;
    game_field[6][3] = 1;
    game_field[6][5] = -2;
    game_field[6][7] = 1;

    game_field[7][0] = 1;
    game_field[7][2] = -2;
    game_field[7][4] = 1;
    game_field[7][6] = -2;
    game_field
}
