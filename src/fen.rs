use crate::piece;
// Module for fen related functions

// Decode a fen string into a board array
// Converts chars like 'P' into a piece number identifier to be used in the board array
pub fn decode(fen: &str) -> [i8; 64] {
    let fen_vec: Vec<char> = fen.chars().collect();

    let mut fen_board: [i8; 64] = [0; 64];
    let mut board: [i8; 64] = [0; 64];


    let mut fen_board_square = 0;
    for i in 0..fen_vec.len() {
        let fen_char = fen_vec[i];
        let skip_squares = fen_char as i8 - 48;

        // Skip '/' since they are meaningless
        if fen_char == '/' {
            continue;
        }

        // Prevent indexing outside of array
        if fen_board_square > 63 {
            break;
        }

        if skip_squares > 0 && skip_squares < 9 { // If a number is detected skip that many board squares
            fen_board_square = fen_board_square + skip_squares;
        } else {
            // Convert char piece identifier to numerical piece identifier, then add the number to the fen_board array
            fen_board[usize::try_from(fen_board_square).unwrap()] = piece::info::id_fen_to_id(fen_char, piece::info::Piece::instantiate_all());
            fen_board_square = fen_board_square + 1;
        }
    }

    // Convert square orders, because the fen array is inverted
    for y in 0..8 {
        for x in 0..8 {
            let fen_square: i8 = -1 * {y - 8 + 1} * {8} + x;
            let reg_square = (y * 8) + x;

            board[usize::try_from(reg_square).unwrap()] = fen_board[usize::try_from(fen_square).unwrap()];
        }
    }
    board // return board
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

        let expected_board: [i8; 64] = [
        wr, wn, wb, wq, wk, wb, wn, wr, 
        wp, wp, wp, wp, wp, wp, wp, wp,
        0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,
        0,  0,  0,  0,  0,  0,  0,  0,
        bp, bp, bp, bp, bp, bp, bp, bp,
        br, bn, bb, bq, bk, bb, bn, br,  
        ];

        let result = decode(fen);
        println!("{}", expected_board[0]);
        assert_eq!(result, expected_board);
    }
}