use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use game_engine::traits::{ChildStates, ScoreOfState, TerminalState};

use super::actions::{Move, Position};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TicTacToeGame {
    pub game_field: [[isize; 3]; 3],
}

impl ScoreOfState for TicTacToeGame {
    fn score_of(&self) -> isize {
        for i in 0..3 {
            if self.winning_row(i) {
                return 10 * self.game_field[i][0];
            }
            if self.winning_column(i) {
                return 10 * self.game_field[0][i];
            }
        }
        if self.game_field[0][0] == self.game_field[1][1]
            && self.game_field[0][0] == self.game_field[2][2]
            || self.game_field[2][0] == self.game_field[1][1]
                && self.game_field[2][0] == self.game_field[0][2]
                && self.game_field[1][1] != 0
        {
            return 10 * self.game_field[1][1];
        }
        0
    }
}
impl Add<Move> for TicTacToeGame {
    type Output = TicTacToeGame;

    fn add(self, rhs: Move) -> Self::Output {
        let mut board = self;
        let sign = match rhs.color {
            true => 1,
            false => -1,
        };
        board.game_field[rhs.pos.x][rhs.pos.y] = sign;
        board
    }
}
impl Sub<Move> for TicTacToeGame {
    type Output = TicTacToeGame;

    fn sub(self, rhs: Move) -> Self::Output {
        let mut board = self;
        board.game_field[rhs.pos.x][rhs.pos.y] = 0;
        board
    }
}
impl ChildStates<Move> for TicTacToeGame {
    fn child_states(&self, color: bool) -> Vec<Move> {
        let mut children = Vec::new();
        for row in 0..3 {
            for column in 0..3 {
                if self.game_field[row][column] == 0 {
                    let new_move = Move {
                        color,
                        pos: Position { x: row, y: column },
                    };
                    children.push(new_move);
                }
            }
        }
        children
    }
}

impl TerminalState for TicTacToeGame {
    fn is_terminal(&self) -> bool {
        let mut found = false;
        for i in 0..3 {
            for j in 0..3 {
                if self.game_field[i][j] == 0 {
                    found = true;
                }
            }
        }
        if !found {
            return true;
        }
        for index in 0..3 {
            if self.winning_row(index) || self.winning_column(index) {
                return true;
            }
        }
        (self.game_field[0][0] == self.game_field[1][1]
            && self.game_field[0][0] == self.game_field[2][2]
            || self.game_field[2][0] == self.game_field[1][1]
                && self.game_field[2][0] == self.game_field[0][2])
            && self.game_field[1][1] == 1
            || self.game_field[1][1] == -1
    }
}

impl TicTacToeGame {
    fn winning_column(&self, column: usize) -> bool {
        self.same_column(column) && self.game_field[0][column] != 0
    }

    fn same_column(&self, column: usize) -> bool {
        self.game_field[0][column] == self.game_field[1][column]
            && self.game_field[0][column] == self.game_field[2][column]
    }
    fn winning_row(&self, row: usize) -> bool {
        self.same_row(row) && self.game_field[row][0] != 0
    }
    fn same_row(&self, row: usize) -> bool {
        self.game_field[row][0] == self.game_field[row][1]
            && self.game_field[row][0] == self.game_field[row][2]
    }
}

impl Display for TicTacToeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{} ", get_char(self.game_field[i][j])).unwrap()
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
const fn get_char(x: isize) -> char {
    match x {
        1 => 'X',
        -1 => 'O',
        _ => '_',
    }
}
