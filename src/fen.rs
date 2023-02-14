use crate::piece;
use crate::board::BOARD_SIZE;
// Module for fen related functions

// Decode a fen string into a board array
// Converts chars like 'P' into a piece number identifier to be used in the board array
pub fn decode(fen: &str) ->  [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
    let fen_vec: Vec<char> = fen.chars().collect();
    let mut board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];

    // Get each character of the fen_vec and add it to the board array
    let mut x = 0;
    let mut y = BOARD_SIZE[1] - 1;

    for i in 0..fen_vec.len() {
        let fen_char = fen_vec[i];
        let skip_num = crate::char_to_num(fen_char); // When a number is in a fen string it indicates how many squares to skip on that row

        // Unwrap skip_num and check it is valid
        let mut valid_skip = true;
        let skip_num = match skip_num {
            Ok(num) => num,
            Err(_) => {valid_skip = false; 0},
        };

        // Ignore '/' char in fen, as it is purely visual, it could be used to fix improper formatting
        // But there is no reason to do this since this function will not take user input
        if fen_char != '/' {

            // If the skip is valid add skip_num to x
            if valid_skip {
                x = x + usize::try_from(skip_num).unwrap();
            } else { // Otherwise convert the fen_char into a piece id and put it on the board
                board[x][y] = piece::info::id_fen_to_id(fen_char, piece::info::Piece::instantiate_all());
                x = x + 1;
            }
        }

        // Reset x and increment y once the x coordinate goes out of bounds
        if x >= BOARD_SIZE[0] {
            x = 0;

            // Break loop before y goes out of bounds
            if y == 0 {
                break;
            }

            y = y - 1;
        }

    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_decode_test() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        // Get piece ids
        // White pieces
        let wp = piece::info::IDS[0];
        let wr = piece::info::IDS[1];
        let wn = piece::info::IDS[2];
        let wb = piece::info::IDS[3];
        let wq = piece::info::IDS[4];
        let wk = piece::info::IDS[5];

        // Black pieces
        let bp = wp * -1;
        let br = wr * -1;
        let bn = wn * -1;
        let bb = wb * -1;
        let bq = wq * -1;
        let bk = wk * -1;

        let mut expected_board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];

        // White bottom row
        expected_board[0][0] = wr;
        expected_board[1][0] = wn;
        expected_board[2][0] = wb;
        expected_board[3][0] = wq;
        expected_board[4][0] = wk;
        expected_board[5][0] = wb;
        expected_board[6][0] = wn;
        expected_board[7][0] = wr;

        // White pawn row
        expected_board[0][1] = wp;
        expected_board[1][1] = wp;
        expected_board[2][1] = wp;
        expected_board[3][1] = wp;
        expected_board[4][1] = wp;
        expected_board[5][1] = wp;
        expected_board[6][1] = wp;
        expected_board[7][1] = wp;

        // Black top row
        expected_board[0][7] = br;
        expected_board[1][7] = bn;
        expected_board[2][7] = bb;
        expected_board[3][7] = bq;
        expected_board[4][7] = bk;
        expected_board[5][7] = bb;
        expected_board[6][7] = bn;
        expected_board[7][7] = br;

        // Black pawn row
        expected_board[0][6] = bp;
        expected_board[1][6] = bp;
        expected_board[2][6] = bp;
        expected_board[3][6] = bp;
        expected_board[4][6] = bp;
        expected_board[5][6] = bp;
        expected_board[6][6] = bp;
        expected_board[7][6] = bp;

        let result = decode(fen);
        assert_eq!(result, expected_board);
    }
}