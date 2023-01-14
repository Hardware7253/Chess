use crate::board::BOARD_SIZE;

pub mod info {
    // Piece ids, order must not change
    // Order                  P  R  N  B  Q  K
    pub const IDS: [i8; 6] = [1, 2, 3, 4, 5, 6];

    #[derive(Debug, Copy, Clone)]
    pub struct Piece {
        pub id_fen: char,
        pub id: i8,   

        pub value: i8,

        pub sliding: bool,

        
        pub mdirs: [[i8; 2]; 8], // Move directions array
        pub mdir_no: i8, // How many move directions there are

        // Non conditional capture directions
        pub mdirs_cap: Option<[[i8; 2]; 2]>, // Move directions just for capturing a piece

        // Conditional capture
        // All of these fields have to be used for a conditional capture

        // Only capture when an enemy piece occupies a square in this array
        // When a sub array is found that has an enemy occupying it the piece perfoming the capture moves to a square defined in mdirs_cap
        // The number that indexed the sub array for condition_adj, is reused for mdirs_cap when finding the movement direction for the capture
        // So the condition in condition_adj and direction in mdirs_cap have to correspond
        pub condition_adj: Option<[[i8; 2]; 2]>, 
        pub condition_self_y: Option<i8>, // Condition for what y coordinate the piece performing the capture has to be at
        pub condition_subj_moves: Option<i8>, // Condition for how many moves the enemy piece found from conition_adj has to have made
    }

    // All pieces use white id and id_fen by default
    // To get black counterparts multiply id by -1 or get lowercase of id_fen
    impl Piece {
        fn pawn() -> Self {
            Piece {
                id_fen: 'P',
                id: IDS[0],

                value: 1,

                sliding: false,

                mdirs: [
                    [0, 1], // Move up
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                ],
                mdir_no: 1,

                mdirs_cap: Some([
                    [1, 1], // Capture up right
                    [-1, 1], // Capture up left
                ]),

                // Capture conditions for en passant
                condition_adj: Some([
                    [1, 0],  // To capture right a piece must be 1 square to the right
                    [-1, 0], // To capture left a piece must be 1 square to the left
                ]),
                
                // Additional en passant conditions
                condition_self_y: Some(4), // The piece must be at y = 4
                condition_subj_moves: Some(1), // The piece being captured must have only moved once
            }
        }

        fn rook() -> Self {
            Piece {
                id_fen: 'R',
                id: IDS[1],

                value: 5,

                sliding: true,

                mdirs: [
                    [1, 0], // Move right
                    [-1, 0], // Move left
                    [0, 1], // Move up
                    [0, 1], // Move down
                    [0, 0], 
                    [0, 0],
                    [0, 0],
                    [0, 0],
                ],
                mdir_no: 4,

                mdirs_cap: None,
                condition_adj: None,
                condition_self_y: None,
                condition_subj_moves: None,
            }

        }

        fn knight() -> Self {
            Piece {
                id_fen: 'N',
                id: IDS[2],

                value: 3,

                sliding: false,

                mdirs: [
                    [1, 2],
                    [2, 1],
                    [1, -2],
                    [-1, 2],
                    [2, -1],
                    [-2, 1],
                    [-2, -1],
                    [-1, -2],
                ],
                mdir_no: 8,

                mdirs_cap: None,
                condition_adj: None,
                condition_self_y: None,
                condition_subj_moves: None,
            }
        }

        fn bishop() -> Self {
            Piece {
                id_fen: 'B',
                id: IDS[3],

                value: 3,

                sliding: true,

                mdirs: [
                    [1, 1], // Move up right
                    [1, -1], // Move down right
                    [-1, 1], // Move up left
                    [-1, -1], // Move down left
                    [0, 0],
                    [0, 0],
                    [0, 0],
                    [0, 0],
                ],
                mdir_no: 4,

                mdirs_cap: None,
                condition_adj: None,
                condition_self_y: None,
                condition_subj_moves: None,
            }
        }

        fn queen() -> Self {
            Piece {
                id_fen: 'Q',
                id: IDS[4],

                value: 9,

                sliding: true,

                mdirs: [
                    [1, 0], // Move right
                    [-1, 0], // Move left
                    [0, 1], // Move up
                    [0, 1], // Move down
                    [1, 1], // Move up right
                    [1, -1], // Move down right
                    [-1, 1], // Move up left
                    [-1, -1], // Move down left
                ],
                mdir_no: 8,

                mdirs_cap: None,
                condition_adj: None,
                condition_self_y: None,
                condition_subj_moves: None,
            }
        }

        fn king() -> Self {
            Piece {
                id_fen: 'K',
                id: IDS[5],

                value: 127,

                sliding: false,

                mdirs: [
                    [1, 0], // Move right
                    [-1, 0], // Move left
                    [0, 1], // Move up
                    [0, 1], // Move down
                    [1, 1], // Move up right
                    [1, -1], // Move down right
                    [-1, 1], // Move up left
                    [-1, -1], // Move down left
                ],
                mdir_no: 8,

                mdirs_cap: None,
                condition_adj: None,
                condition_self_y: None,
                condition_subj_moves: None,
            }
        }

        // Instantiates all pieces and returns them in an array
        // The array is sorted so that when indexed by an id - 1 that array element will be the corresponding piece
        pub fn instantiate_all() -> [Piece; 6] {
            let p = Piece::pawn();
            let r = Piece::rook();
            let n = Piece::knight();
            let b = Piece::bishop();
            let q = Piece::queen();
            let k = Piece::king();

            // Do not tamper
            let piece_array_def = [p, r, n, b, q, k];
            let mut piece_array = piece_array_def;

            // Sorts piece_vec_def to piece_vec
            // Sorts in a way that ensures when indexing piece_vec with piece id - 1 it gives the corresponding struct to that id
            for i in 0..IDS.len() {
                piece_array[usize::try_from(IDS[i] - 1).unwrap()] = piece_array_def[i];
            }

            piece_array
        }
    }

    // Convert id_fen to id
    // Function borrows pieces array rather than instantiating it each time
    // This allows the function to be only ~7x slower than a hardcoded match statement, instead of ~60x (over thousand of consecutive iterations)
    pub fn id_fen_to_id(mut id_fen: char, pieces: [Piece; 6]) -> i8 {

        // Check if piece is white
        let mut white = true;
        if !crate::uppercase_char(id_fen) {
            id_fen = crate::lower_to_upper_char(id_fen); // Convert char to uppercase since instances in pieces array use uppercase ids
            white = false;
        }

        // Find index to piece in piece array which matches id_fen
        let mut pieces_index = 0;
        for i in 0..pieces.len() {
            if pieces[i].id_fen == id_fen {
                pieces_index = i;
            }
        }


        let mut id = pieces[pieces_index].id; // Get id
        if !white { // Convert white id to black id if neccessary
            id = id * -1;
        }
        id
    }

    // Return true if given piece id is a pawn
    pub fn piece_pawn(id: i8) -> bool {
        let pawn = Piece::pawn();
        if pawn.id == id || pawn.id == id * -1 {
            return true;
        }
        false
    }

    // Return true if given piece id is a knight
    pub fn piece_knight(id: i8) -> bool {
        let knight = Piece::pawn();
        if knight.id == id || knight.id == id * -1 {
            return true;
        }
        false
    }
}


pub mod moves {
    use super::*;


    // Generates all possible moves given a single piece
    pub fn gen_moves(piece_coordinate: [i8; 2], board: [[i8; 8]; 8], turns_board: [[u16; 8]; 8], pieces: [info::Piece; 6]) -> [[i8; 8]; 8] {

        // Retrieve additional information on the piece
        let id = board[usize::try_from(piece_coordinate[0]).unwrap()][usize::try_from(piece_coordinate[1]).unwrap()];
        let mdirs = pieces[usize::try_from(id).unwrap()].mdirs;
        let slides = pieces[usize::try_from(id).unwrap()].sliding;
        
        // Special captures and conditions
        let mdirs_cap = pieces[usize::try_from(id).unwrap()].mdirs_cap;
        let condition_adj = pieces[usize::try_from(id).unwrap()].condition_adj;
        let condition_self_y = pieces[usize::try_from(id).unwrap()].condition_self_y;
        let condition_subj_moves = pieces[usize::try_from(id).unwrap()].condition_subj_moves;

        // Check for special capture
        let mut special_capture = false;
        match mdirs_cap {
            Some(_) => special_capture = false,
            None => special_capture = true,
        }

        // Check for conditional capture
        let mut conditional_capture = false;
        match condition_adj {
            Some(_) => conditional_capture = false,
            None => conditional_capture = true,
        }

        // Unwrap Option<T> struct fields
        let mdirs_cap = mdirs_cap.unwrap_or([[0i8; 2]; 2]);
        let condition_adj = condition_adj.unwrap_or([[0i8; 2]; 2]);
        let condition_self_y = condition_self_y.unwrap_or(0);
        let condition_subj_moves = condition_subj_moves.unwrap_or(0);

        for x in 0..BOARD_SIZE[0] {
            for y in 0..BOARD_SIZE[1] {
                
            }
        }

        [[0i8; 8]; 8]

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
mod tests {
    use super::*;
    
    mod info_tests {
        use super::*;

        #[test]
        fn id_fen_to_id() {
            let pieces = info::Piece::instantiate_all();
            assert_eq!(info::id_fen_to_id('p', pieces), info::IDS[0] * -1);
        }
    }

    mod move_tests {
        use super::*;


    }
}