pub mod fen;
pub mod piece;
pub mod board;
use crate::board::BOARD_SIZE;

// Converts chess coordinate notation (a2) to cartesian coordinates (x, y)
pub fn ccn_to_cart(ccn: Vec<char>) -> Result<[i8; 2], ()> {

    // Turn chars into their respective ASCII values
    let x = ccn[0] as i8 - 97;
    let y = ccn[1] as i8 - 49;

    // Get board max coordinates
    let bx = BOARD_SIZE[0] - 1;
    let by = BOARD_SIZE[1] - 1;

    let bx: i8 = bx.try_into().unwrap();
    let by: i8 = by.try_into().unwrap();

    // Return an error if the converted coordinates are outside the range of a typical chess board
    if x > bx || x < 0 || y > by || y < 0 {
        return Err(())
    }

    let cart: [i8; 2] = [x, y];
    Ok(cart)
}

// Return true if a char is uppercase
pub fn uppercase_char(c: char) -> bool {
    let c_num = c as i8;
    if c_num > 90 || c_num < 65 {
        return false;
    }
    true
}

// Convert lowercase char to uppercase char
pub fn lower_to_upper_char(c: char) -> char {
    let c_upper = c as u8 - 32;
    if c_upper  > 90 || c_upper < 65 {
        panic!("Lower case char not provided");
    }
    c_upper as char
}

// Convert a char of a number to an integer
// E.g. '1' -> 1
pub fn char_to_num(c: char) -> Result<i8, ()> {
    let num = c as i8 - 48;
    if num < 0 || num > 9 {
        return Err(())
    }
    Ok(num)
}

// Unwrap an Option<T>, if there is not a Some() value return the def parameter
pub fn unwrap_def<T>(option: Option<T>, def: T) -> T {
    match option {
        Some(t) => t,
        None => def,
    }
}

// Flip board so that the coordinates match the perspective of the other player
pub fn invert_board(board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]) -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
    let mut board_inv = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];

    for x in 0..BOARD_SIZE[0] {
        for y in 0..BOARD_SIZE[1] {

            // Get x and y as integers
            let invx: i16 = x.try_into().unwrap();
            let invy: i16 = y.try_into().unwrap();

            // Get board size in integers
            let boardx: i16 = BOARD_SIZE[0].try_into().unwrap();
            let boardy: i16 = BOARD_SIZE[1].try_into().unwrap();

            // Invert coordinates
            let invx: i16 = invx - {boardx - 1};
            let invy: i16 = invy - {boardy - 1};
            let invx = invx.abs();
            let invy = invy.abs();
            
            // Get inverted coordiantes back into usize
            let invx: usize = usize::try_from(invx).unwrap();
            let invy: usize = usize::try_from(invy).unwrap();

            board_inv[invx][invy] = board[x][y];
        }
    }
    board_inv
}

// Check if a given coordinates is valid on the chess board
pub fn fits_in_board(coordinates: [i8; 2]) -> bool {
    if (coordinates[0] >= 0) && (coordinates[0] < BOARD_SIZE[0].try_into().unwrap()) && (coordinates[1] >= 0) && (coordinates[1] < BOARD_SIZE[1].try_into().unwrap()) {
        return true;
    }
    false
}

// Returns the value at a given coordinates on a board array
pub fn get_board(coordinates: [i8; 2], board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]) -> i8 {
    board[usize::try_from(coordinates[0]).unwrap()][usize::try_from(coordinates[1]).unwrap()]
}

// Returns the original board, with the value at coordinates
pub fn set_board(coordinates: [i8; 2], value: i8, mut board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]) -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
    board[usize::try_from(coordinates[0]).unwrap()][usize::try_from(coordinates[1]).unwrap()] = value;
    board
}

// Moves the value corresponding from original_coordinates to new_coordinates on the board
// Replaces original_coordinates value with default_value
fn move_board_value(original_coordinates: [i8; 2], new_coordinates: [i8; 2], default_value: i8, mut board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]) -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
    let value = get_board(original_coordinates, board);
    board = set_board(new_coordinates, value, board); // Move value to new coordinates
    board = set_board(original_coordinates, default_value, board); // Remove value from original coordinates
    board
}

// Detects friendly pieces given 2 piece ids
pub fn friendly_piece(p1: i8, p2: i8) -> bool {
    if p1 * p2 > 0 {
        true
    } else {
        false
    }
}

// Returns true if a piece is white
pub fn piece_white(id: i8) -> bool {
    if id < 0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::piece::info;
    use super::*;

    #[test]
    fn ccn_to_cart_test() {
        let pos_vec = vec!['a', '2'];
        let result = ccn_to_cart(pos_vec).unwrap();

        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn uppercase_char_test() {
        assert_eq!(uppercase_char('A'), true);
    }

    #[test]
    fn lower_to_upper_char_test() {
        assert_eq!(lower_to_upper_char('a'), 'A');
    }

    #[test]
    fn char_to_num_test() {
        assert_eq!(char_to_num('0'), Ok(0));
    }

    #[test] 
    fn unwrap_def_test() {
        let num: i8 = 2;
        let num_option: Option<i8> = Some(num);

        assert_eq!(unwrap_def(num_option, 0), num);
    }

    #[test]
    fn invert_board_test() {
        let board = fen::decode("rrrr4/rrrrpp2/1p1ppnq1/1ppp2q1/3P4/2PP4/BQ1PPP2/QQ3KRR");
        let expected = fen::decode("RRK3QQ/2PPP1QB/4PP2/4P3/1q2ppp1/1qnpp1p1/2pprrrr/4rrrr");
        assert_eq!(invert_board(board), expected);
    }

    #[test]
    fn fits_in_board_test() {
        assert_eq!(fits_in_board([0, -1]), false);
    }

    #[test]
    fn get_board_test() {
        let board = fen::decode("8/8/8/8/3P4/8/8/8");
        let wp = info::IDS[0];

        assert_eq!(get_board([3, 3], board), wp);
    }

    #[test]
    fn set_board_test() {
        let board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];
        let result = set_board([3, 3], 1, board);

        let expected = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];

        assert_eq!(result, expected);
    }

    #[test]
    fn move_board_value_test() {
        let board = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
        let expected = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];

        let result = move_board_value(
            [3, 3],
            [3, 4],
            0,
            board,
        );

        assert_eq!(result, expected);
    }

    #[test]
    fn friendly_piece_test() {
        assert_eq!(friendly_piece(1, -2), false);
    }

    #[test]
    fn piece_white_test() {
        assert_eq!(piece_white(-1), false);
    }
}
