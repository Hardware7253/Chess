

// Module containing piece information
pub mod info {

    // Convert char id to num id
    pub fn char_to_num_id(char_id: char) -> i8 {
        match char_id {
            'p' => -1,
            'r' => -2,
            'n' => -3,
            'b' => -4,
            'q' => -5,
            'k' => -6,
            'P' =>  1,
            'R' =>  2,
            'N' =>  3,
            'B' =>  4,
            'Q' =>  5,
            'K' =>  6,
            other => 0,
        }
    }

    // Convert num id to char id
    pub fn num_to_char_id(num_id: i8) -> char {
        match num_id {
            -1 => 'p',
            -2 => 'r',
            -3 => 'n',
            -4 => 'b',
            -5 => 'q',
            -6 => 'k',
            01 => 'P',
            02 => 'R',
            03 => 'N',
            04 => 'B',
            05 => 'Q',
            06 => 'K',
            other => '0',
        }
    }

    // Convert num id to piece movement directions
    pub fn num_id_to_move_dirs(mut num_id: i8) -> [i8; 8] {
        if num_id < 0 {
            num_id = num_id * -1;
        }
        match num_id {
            2 => [1, -1, 8, -8, 0, 0, 0, 0],         // Rook
            4 => [9, -9, 7, -7, 0, 0, 0, 0],         // Bishop
            5 => [1, -1, 8, -8, 9, -9, 7, -7],       // Queen
            6 => [1, -1, 8, -8, 9, -9, 7, -7],       // King
            other => [0; 8],
            // Knight and pawn capture moves are in generate_knight_moves and calculate_pawn_captures respectively
            // Because both those types of moves are more specialist and need other conditions which cannot be stored in [i8; 8] format
        }
    }

    // Convert num id to a boolean which states wether the piece can move multiple squares or not
    pub fn num_id_to_sliding(mut num_id: i8) -> bool {
        if num_id < 0 {
            num_id = num_id * -1;
        }
        match num_id {
            1 => false, // Pawn
            2 => true,  // Rook
            3 => false, // Knight
            4 => true,  // Bishop
            5 => true,  // Queen
            6 => false, // King
            other => false,
        }
    }

    // Convert num id to piece values
    pub fn num_id_to_piece_value(mut num_id: i8) -> i8 {
        if num_id < 0 {
            num_id = num_id * -1;
        }
        match num_id {
            1 => 1,   // Pawn
            2 => 5,   // Rook
            3 => 3,   // Knight
            4 => 3,   // Bishop
            5 => 8,   // Queen
            6 => 127, // King
            other => 0
        }
    }

    // Return true if given piece id is a pawn
    pub fn piece_pawn(id: i8) -> bool {
        if id == 1 || id == -1 {
            return true;
        }
        false
    }

    // Return true if given piece id is a knight
    pub fn piece_knight(id: i8) -> bool {
        if id == 3 || id == -3 {
            return true;
        }
        false
    }
}

// Module containing functions which generate possible moves for a piece
pub mod moves {
    use crate::piece;

    // Generates possible squares a piece can move to, accounting for paths blocked by friendly pieces, and captures of enemy pieces
    pub fn generate_move_squares(piece_coordinate: i8, board: [i8; 64], turns_board: [u16; 64]) -> [i8; 64] {

        // Retrieve additional information on the piece at x coordinate on the board
        let piece_id = board[usize::try_from(piece_coordinate).unwrap()];
        let piece_movement_directions = piece::info::num_id_to_move_dirs(piece_id);
        let piece_slides = piece::info::num_id_to_sliding(piece_id);


        let mut movement_squares: [i8; 64] = [0; 64];

        // Get piece movement for all sliding pieces
        if piece_slides {
            let mut indexer = 0;
            for i in 0..8 {

                let mut break_j_on = 0;
                for j in 1..8 {
                    if j == break_j_on {
                        break
                    }

                    let move_square: i8 = piece_coordinate + piece_movement_directions[i] * j;

                    if valid_square(move_square) { // Check the move square is valid
                        if square_occupied(move_square, board) { // Check for pieces in the move_square
                            if friendly_piece(piece_id, board[usize::try_from(move_square).unwrap()]) { // Detect if a friendly piece is in the move_square
                                break // Stop moving in that direction since there is a piece in the way
                            } else {
                                break_j_on = j + 1; // Stop moving in that direction one step later so the enemy piece can be captured
                            }
                        }
                        
                        movement_squares[usize::try_from(move_square).unwrap()] = 1;
                    }
                    indexer = indexer + 1;
                }
            }
        }

        // Get piece movement for knights
        if piece::info::piece_knight(piece_id) {
            return generate_knight_moves(piece_coordinate, board);
        }

        // Get piece movement for pawns
        if piece::info::piece_pawn(piece_id) {
            return generate_pawn_moves(piece_coordinate, board, turns_board);
        }

        movement_squares
    }


    // Generates squars a pawn can move to
    // Accounts for enemy piece captures including en passant
    // An extra board is required for this function that stores how many turns each piece has made
    fn generate_pawn_moves(piece_coordinate: i8, board: [i8; 64], turns_board: [u16; 64]) -> [i8; 64] {
        let piece_id = board[usize::try_from(piece_coordinate).unwrap()];
        if !piece::info::piece_pawn(piece_id) {
            return [0i8; 64];
        }

        let pawn_movement_directions: [i8; 2] = [8, 0];
        let mut pawn_capture_directions = [0i8; 2];
        let mut en_passant_condition_piece = [0i8; 2];

        // Add pawn capture directions to array
        // Exclude directions that would be outside of the board for this piece_coordinate
        if piece_coordinate % 8 != 0 { // Prevent left movements if on the far left side
            pawn_capture_directions[0] = 7;
            en_passant_condition_piece[0] = -1;
        } if piece_coordinate - 7 % 8 != 0 { // Prevent right movement if on the far right side
            pawn_capture_directions[1] = 9;
            en_passant_condition_piece[1] = 1;
        }
        
        let mut movement_squares = [0i8; 64];
        
        for i in 0..2 {
            let move_square = piece_coordinate + pawn_movement_directions[i];
            let move_square_cap = piece_coordinate + pawn_capture_directions[i];
            let en_passant_condition_square = piece_coordinate + en_passant_condition_piece[i];

            // Check if the pawn can move
            if valid_square(move_square) && !square_occupied(move_square, board) {
                movement_squares[usize::try_from(move_square).unwrap()] = 1;
            }

            // Check if a piece can be captured
            if valid_square(move_square_cap) && square_occupied(move_square_cap, board) {
                if !friendly_piece(piece_id, board[usize::try_from(move_square_cap).unwrap()]) {
                    movement_squares[usize::try_from(move_square_cap).unwrap()] = 1;
                }
            }

            // Check if a piece can be captured via en passant
            if valid_square(move_square_cap) && square_occupied(en_passant_condition_square, board) && piece_coordinate / 8 == 4 { // The condition square has to be occupied by a piece, and the piece coordinate has to be in the 3rd row from the top of the board
                if !friendly_piece(piece_id, board[usize::try_from(en_passant_condition_square).unwrap()]) { // The condition square has to be occupied by an enemy piece
                    if turns_board[usize::try_from(en_passant_condition_square).unwrap()] == 1 { // Finally the enemy piece has to have only moved once

                        // En passant is valid
                        movement_squares[usize::try_from(move_square_cap).unwrap()] = 1; // Set square that the pawn moves to 1
                        movement_squares[usize::try_from(en_passant_condition_square).unwrap()] = -1; // Set the square of the enemy pieces capture to -1 to indicate that it needs to be deleted
                    } 
                }
            }
        }
        movement_squares
    }

    // Generates squares a knight can move to
    fn generate_knight_moves(piece_coordinate: i8, board: [i8; 64]) -> [i8; 64] {
        let piece_id = board[usize::try_from(piece_coordinate).unwrap()];
        if !piece::info::piece_knight(piece_id) {
            return [0i8; 64];
        }

        let mut conversion_array = [[0i8; 8]; 8]; // 2D Array used for mapping board square values for conversion
        let mut movement_squares = [0i8; 64];
        let mut piece_coordinate_c = [0i8; 2];

        // Knight moveset in cartesian
        // Because large jumps like the knight does causes errors with the standard coordinate system
        // Usaully when a knight is near one of the board edges
        let knight_moves_c: [[i8; 2]; 8] = [
          // y, x
            [1, 2],
            [2, 1],
            [1, -2],
            [-1, 2],
            [2, -1],
            [-2, 1],
            [-2, -1],
            [-1, -2],
        ];

        // Convert piece board coordinate into cartesian coordinate
        // Additionally make an array with the board coordinates, that can be indexed by cartesian coordinates
        // This makes converting back later easier
        let mut total_iters = 0;
        for y in 0..8 {
            for x in 0..8 {
                conversion_array[y][x] = total_iters;
                if piece_coordinate == total_iters {
                    piece_coordinate_c = [y.try_into().unwrap(), x.try_into().unwrap()];
                }
                total_iters = total_iters + 1;
            }
        }

        println!("{}", conversion_array[3][4]);

        // Get knight moves, check for validity, and write possible moves to movement_squares array
        for i in 0..8 {
            let moved_coordinate: [i8; 2] = [piece_coordinate_c[0] + knight_moves_c[i][0], piece_coordinate_c[1] + knight_moves_c[i][1]]; // Get coordinate after the night move
            if check_fits_2d_array(moved_coordinate, [8, 8]) { // If the coordinate fits in an 8*8 array it is valid
                let move_square = conversion_array[usize::try_from(moved_coordinate[0]).unwrap()][usize::try_from(moved_coordinate[1]).unwrap()]; // Convert back to board coordinate

                // If there is an enemy at the move sqaure or if the move sqaure is unoccupied the knight can move there
                if !friendly_piece(piece_id, board[usize::try_from(move_square).unwrap()]) || !square_occupied(move_square, board) {
                    movement_squares[usize::try_from(move_square).unwrap()] = 1;
                }
            }
        }
        movement_squares
    }

    // Check if a given coordinate will work in a 2d array of specified size
    fn check_fits_2d_array(coordinate: [i8; 2], array_size: [i8; 2]) -> bool {
        if (coordinate[1] >= 0) && (coordinate[1] < array_size[1]) && (coordinate[0] >= 0) && (coordinate[0] < array_size[0]) {
            return true;
        }
        false
    }

    // Detects friendly pieces given 2 piece number ids
    fn friendly_piece(p1: i8, p2: i8) -> bool {
        if p1 * p2 > 0 {
            true
        } else {
            false
        }
    }

    // Detects if a square is part of the board or not
    fn valid_square(square: i8) -> bool {
        if square < 64 && square > -1 {
            true
        } else {
            false
        }
    }

    // Checks if a square on the board is occupied by a piece
    fn square_occupied(square: i8, board: [i8; 64]) -> bool {
        if board[usize::try_from(square).unwrap()] != 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod piece_moves_tests {
    use crate::piece;
    use crate::fen;

    #[test]
    fn sliding_piece_moves() { // Test move square generation, with a sliding piece and no pieces to capture
        let board = fen::decode("7k/8/8/8/P7/8/8/R1P3K1");
        let expected_result = [0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = piece::moves::generate_move_squares(0, board, [0u16; 64]);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn sliding_piece_moves_c() { // Test move square generation, with a sliding piece and pieces to capture
        let board = fen::decode("kK6/8/8/8/b7/8/8/R1b5");
        let expected_result = [0, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = piece::moves::generate_move_squares(0, board, [0u16; 64]);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn knight_moves() { // Test a knight capturing two bishops
        let board = fen::decode("K6k/8/8/8/8/1b6/2b5/N7");
        let expected_result = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = piece::moves::generate_move_squares(0, board, [0u16; 64]);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn en_passant_test() { // Test a pawn capturing a piece with en passant
        let board = fen::decode("8/8/8/5pP1/8/8/8/8");
        let expected_result = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let turns_board = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let result = piece::moves::generate_move_squares(38, board, turns_board);

        assert_eq!(result, expected_result);
    }
}