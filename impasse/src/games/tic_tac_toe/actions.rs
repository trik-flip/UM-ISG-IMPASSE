pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Move {
    pub pos: Position,
    pub color: bool,
}
impl Default for Move {
    fn default() -> Self {
        Move {
            pos: Position { x: 0, y: 0 },
            color: true,
        }
    }
}
