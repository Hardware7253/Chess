// Board size is able to be changed, unit tests are likely to break
pub const BOARD_SIZE: [usize; 2] = [8, 8];
pub const MAX_SLIDES: usize = 8; // Maximum number of squares a piece should be able to move to, this is equal to the longest side of the board

pub mod errors {
    pub const CHECK_ERROR: i8 = 1;
    pub const CHECKMATE_ERROR: i8 = 2;
    pub const STALEMATE_ERROR: i8 = 3;
    pub const INVALID_MOVE_ERROR: i8 = 4;
}