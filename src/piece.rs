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

        // All mdirs arrays have to be indexed like [y][x], whereas boards are indexed [x][y]
        // This is because mdirs readability is important
        pub mdirs: [[i8; 2]; 8], // Move directions array
        pub mdir_no: usize, // How many move directions there are, if a piece has speical capture directions there can be a max of 2 mdirs

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
                    [0, -1], // Move down
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
                    [0, -1], // Move down
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
                    [0, -1], // Move down
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
    pub fn id_fen_to_id(mut id_fen: char, pieces: [Piece; 6]) -> i8 {
        //let mut id: i8 = 0;

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
    use crate::unwrap_def;


    // Generates all possible moves given a single piece
    pub fn gen_moves(
        piece_coordinate: [i8; 2],
        board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
        turns_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
        pieces: [info::Piece; 6])
        -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {

        // Retrieve additional information on the piece
        let id = board[usize::try_from(piece_coordinate[0]).unwrap()][usize::try_from(piece_coordinate[1]).unwrap()];
        let mdirs = pieces[usize::try_from(id - 1).unwrap()].mdirs;
        let mut mdir_no = pieces[usize::try_from(id - 1).unwrap()].mdir_no;
        let slides = pieces[usize::try_from(id - 1).unwrap()].sliding;
        
        // Special captures and conditions
        let mdirs_cap = pieces[usize::try_from(id - 1).unwrap()].mdirs_cap;
        let condition_adj = pieces[usize::try_from(id - 1).unwrap()].condition_adj;
        let condition_self_y = pieces[usize::try_from(id - 1).unwrap()].condition_self_y;
        let condition_subj_moves = pieces[usize::try_from(id - 1).unwrap()].condition_subj_moves;

        // Check for special capture
        let mut special_capture = false;
        match mdirs_cap {
            Some(_) => {
                special_capture = true;
                mdir_no = 2;
            },

            None => special_capture = false,
        };

        // Check for conditional capture
        let mut conditional_capture = false;
        match condition_adj {
            Some(_) => conditional_capture = true,
            None => conditional_capture = false,
        };

        /*
        // Unwrap Option<T> struct fields
        let mdirs_cap = mdirs_cap.unwrap_or([[0i8; 2]; 2]);
        let condition_adj = condition_adj.unwrap_or([[0i8; 2]; 2]);
        let condition_self_y = condition_self_y.unwrap_or(0);
        let condition_subj_moves = condition_subj_moves.unwrap_or(0);
        */

        let mdirs_cap = unwrap_def(mdirs_cap, [[0i8; 2]; 2]);
        let condition_adj = unwrap_def(condition_adj, [[0i8; 2]; 2]);
        let condition_self_y = unwrap_def(condition_self_y, 0);
        let condition_subj_moves = unwrap_def(condition_subj_moves, 0);

        let mut moves_board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];

        // If the piece is sliding set repeats to the maximum ammount it needs to be for the current board size
        // Repeats makes one mdir apply repeatedly until it reaches the end of the board
        let mut repeats: usize = 1;
        if slides {
            if BOARD_SIZE[0] > BOARD_SIZE[1] {
                repeats = BOARD_SIZE[0];
            }
            repeats = BOARD_SIZE[1];
        }

        for i in 0..mdir_no {
            let mut piece_coordinate_current = piece_coordinate;
            for j in 0..repeats {
                // Get the pieces coordinate after applying the x and y coordinate change from mdirs array
                let new_coordinate = [
                    piece_coordinate_current[0] + mdirs[i][0],
                    piece_coordinate_current[1] + mdirs[i][1],
                ];

                // Get special capture coordinate when applicable
                let mut new_coordinate_cap = [0i8; 2];
                let mut new_coordinates_len = 1;
                if special_capture && i < 2 {
                    new_coordinate_cap = [
                        piece_coordinate_current[0] + mdirs_cap[i][0],
                        piece_coordinate_current[1] + mdirs_cap[i][1],
                    ];
                    new_coordinates_len = 2;
                }

                // Get conditional capture coordinate when applicable
                if conditional_capture && i < 2 {
                    let new_coordinate_condition = [
                        piece_coordinate_current[0] + condition_adj[i][0],
                        piece_coordinate_current[1] + condition_adj[i][1],
                    ];

                    // Do conditional capture when conditions are met ------------------------------------------------------------------------------------------------------------- En passant broken, check Option<T> unwraps
                    if !friendly_piece(id, index_board(new_coordinate_condition, board)) { // Condition square must be occupied by an enemy piece
                        if index_board(new_coordinate_condition, turns_board) == condition_subj_moves { // Piece in condition square must have moved condition_subj_moves times
                            if piece_coordinate_current[1] == condition_self_y { // Piece performing the special capture must be at y coordinate condition_self_y

                                if valid_move(id, new_coordinate_cap, board, false) {
                                    moves_board[usize::try_from(new_coordinate_cap[0]).unwrap()][usize::try_from(new_coordinate_cap[1]).unwrap()] = 1;
                                }
                            }
                        }
                    }
                }

                let new_coordinates = [new_coordinate, new_coordinate_cap];
                println!("{:?}", new_coordinates);
                //println!("{}", new_coordinates_len);

                let mut force_capture = false;
                for c in 0..new_coordinates_len {
                    println!("{}", c);
                    
                    // Only allow capturing of pieces if the piece doesn't have special capture moves
                    if valid_move(id, new_coordinates[c], board, force_capture) {
                        let new_coordinate_valid = new_coordinates[c];

                        moves_board[usize::try_from(new_coordinate_valid[0]).unwrap()][usize::try_from(new_coordinate_valid[1]).unwrap()] = 1;
                        piece_coordinate_current = new_coordinate_valid;
                        println!("{:?}", piece_coordinate_current);
                    }
                    force_capture = true;
                }
            }
        }
        moves_board
    }

    // Used for validating a new move direction for a piece
    pub fn valid_move(id: i8, new_coordinate: [i8; 2], board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]], force_capture: bool) -> bool {
        if fits_in_board(new_coordinate) {
            let new_coordinate_id =  index_board(new_coordinate, board);

            // The move is valid if the move square is empty
            if new_coordinate_id == 0 && !force_capture {
                return true;
            }

            // The move is valid if the piece in the move square is an enemy
            if !friendly_piece(id, new_coordinate_id) && new_coordinate_id != 0 {
                return true;
            }
        }
        false
    }

    // Check if a given coordinate is valid on the chess board
    pub fn fits_in_board(coordinate: [i8; 2]) -> bool {
        if (coordinate[0] >= 0) && (coordinate[0] < BOARD_SIZE[0].try_into().unwrap()) && (coordinate[1] >= 1) && (coordinate[1] < BOARD_SIZE[1].try_into().unwrap()) {
            return true;
        }
        false
    }

    // Returns the value at a given coordinate on a board array
    pub fn index_board(coordinate: [i8; 2], board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]) -> i8 {
        board[usize::try_from(coordinate[0]).unwrap()][usize::try_from(coordinate[1]).unwrap()]
    }

    // Detects friendly pieces given 2 piece ids
    pub fn friendly_piece(p1: i8, p2: i8) -> bool {
        if p1 * p2 > 0 {
            true
        } else {
            false
        }
    }

}


#[cfg(test)]
mod tests {
    use crate::fen;
    use super::*;

    mod moves_tests {
        use super::*;

        #[test]
        fn gen_moves_test1() { // Test friendly pieces blocking a sliding pieces path
            for i in 0..1 {
                let pieces = info::Piece::instantiate_all();

                let board = fen::decode("8/2B5/8/8/2Q5/2B5/8/5B2");
                let expected: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] = [
                    [0, 1, 0, 1, 0, 1, 0, 0],
                    [0, 0, 1, 1, 1, 0, 0, 0],
                    [0, 0, 0, 0, 1, 1, 0, 0],
                    [0, 0, 1, 1, 1, 0, 0, 0],
                    [0, 1, 0, 1, 0, 1, 0, 0],
                    [0, 0, 0, 1, 0, 0, 1, 0],
                    [0, 0, 0, 1, 0, 0, 0, 1],
                    [0, 0, 0, 1, 0, 0, 0, 0],
                ];

                let move_board = moves::gen_moves(
                    [2, 3],
                    board,
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    pieces,
                );

                assert_eq!(move_board, expected);
            }
        }

        #[test]
        fn gen_moves_test2() { // Test en passant
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/8/7p/5pP1/8/8/8/8");

            let turns_board = [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0], 
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0]
            ];

            let expected: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] = [
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0, 1, 0, 0]
            ];

            /*
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0]
            */

            let move_board = moves::gen_moves(
                [6, 4],
                board,
                turns_board,
                pieces,
            );

            assert_eq!(move_board, expected);
        }

        #[test]
        fn valid_move_test() {
            let board = fen::decode("8/8/8/4p3/3B4/8/8/8");
            let bb = info::IDS[3] * -1;

            assert_eq!(moves::valid_move(bb, [4, 4], board, false), false);
        }

        #[test]
        fn fits_in_board_test() {
            assert_eq!(moves::fits_in_board([0, BOARD_SIZE[1].try_into().unwrap()]), false);
        }

        #[test]
        fn index_board_test() {
            let board = fen::decode("8/8/8/8/3P4/8/8/8");
            let wp = info::IDS[0];

            assert_eq!(moves::index_board([3, 3], board), wp);
        }

        #[test]
        fn friendly_piece_test() {
            assert_eq!(moves::friendly_piece(1, -2), false);
        }
    }
    
    mod info_tests {
        use super::*;

        #[test]
        fn id_fen_to_id() {
            let pieces = info::Piece::instantiate_all();

            for i in 0..1 {
                assert_eq!(info::id_fen_to_id('n', pieces), info::IDS[2] * -1);
            }
            
        }
    }
}