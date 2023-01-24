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
        pub condition_self_y: Option<i8>, // Condition for what y coordinates the piece performing the capture has to be at
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn id_fen_to_id_test() {
            let pieces = Piece::instantiate_all();

            for i in 0..1 {
                assert_eq!(id_fen_to_id('n', pieces), IDS[2] * -1);
            }
            
        }
    }
}


pub mod moves {
    use super::*;

    // Bring library functions into scope
    use crate::unwrap_def;
    use crate::fits_in_board;
    use crate::get_board;
    use crate::set_board;
    use crate::friendly_piece;
    use crate::piece_white;

    // Generates all possible moves for a type of piece (white or black)
    pub fn gen_all_moves(
    board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    turns_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    last_turn_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    pieces: [info::Piece; 6],
    gen_all_white: bool) // When true generates all white moves, generates black mvoes when false
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
    
        let mut moves_board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];
        for x in 0..BOARD_SIZE[0] {
            for y in 0..BOARD_SIZE[1] {
                let id = board[x][y]; // Get id of piece at board coordinates (x, y)

                if piece_white(id) == gen_all_white && id != 0 { // Check piece type matches piece type defined in gen_all_white
                    let piece_coordinates = [x.try_into().unwrap(), y.try_into().unwrap()];
                    println!("{:?}", piece_coordinates);

                    moves_board = gen_moves(piece_coordinates, board, turns_board, moves_board, last_turn_board, pieces);
                    
                }
            }
        }
        moves_board
    }


    // Generates all possible moves given a single piece
    pub fn gen_moves(
    mut piece_coordinates: [i8; 2],
    board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    turns_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    starting_moves_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    last_turn_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    pieces: [info::Piece; 6])
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {

        let mut moves_board = starting_moves_board;
        

        // Get piece id
        let id = get_board(piece_coordinates, board);
        if id == 0 {
            return [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];
        }

        // If the piece has a negative id change it to positive so it can be used to index pieces array
        let mut pieces_index = id;
        if pieces_index < 0 {
            pieces_index = pieces_index * -1;
        }
        pieces_index = pieces_index - 1; // -1 because piece ids start from 1 not 0

        // mdirs information (movement directions)
        let mdirs = pieces[usize::try_from(pieces_index).unwrap()].mdirs;
        let mut mdir_no = pieces[usize::try_from(pieces_index).unwrap()].mdir_no;
        let slides = pieces[usize::try_from(pieces_index).unwrap()].sliding;
        
        // Special captures and conditions
        let mdirs_cap = pieces[usize::try_from(pieces_index).unwrap()].mdirs_cap;
        let condition_adj = pieces[usize::try_from(pieces_index).unwrap()].condition_adj;
        let condition_self_y = pieces[usize::try_from(pieces_index).unwrap()].condition_self_y;
        let condition_subj_moves = pieces[usize::try_from(pieces_index).unwrap()].condition_subj_moves;

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

        // Unwrap Option<T> struct fields
        let mdirs_cap = unwrap_def(mdirs_cap, [[0i8; 2]; 2]);
        let condition_adj = unwrap_def(condition_adj, [[0i8; 2]; 2]);
        let condition_self_y = unwrap_def(condition_self_y, 0);
        let condition_subj_moves = unwrap_def(condition_subj_moves, 0);
        

        // Generate special capture moves
        if special_capture {
            for i in 0..2 {

                // Get capture coordinates
                let capture_coordinates = [
                    piece_coordinates[0] + mdirs_cap[i][0],
                    piece_coordinates[1] + mdirs_cap[i][1],
                ];

                if fits_in_board(capture_coordinates) { // Check capture coordinates fit in the board
                    let capture_coordinates_id = get_board(capture_coordinates, board);
                    if !friendly_piece(id, capture_coordinates_id) && capture_coordinates_id != 0 { // Check there is a piece to capture


                        // Set moves_board to 1 at the capture coordinates to indicate that the piece can move there
                        moves_board = set_board(capture_coordinates, 1, moves_board);
                    }
                }
            } 
        }

        // Generate conditional capture moves
        if conditional_capture {
            for i in 0..2 {

                // Get capture coordinates (coordinates that the piece will move to if all conditions are met)
                let capture_coordinates = [
                    piece_coordinates[0] + mdirs_cap[i][0],
                    piece_coordinates[1] + mdirs_cap[i][1],
                ];

                // Get condition coordinates (coordinates that have to be occupied by an enemy piece)
                let condition_coordinates = [
                    piece_coordinates[0] + condition_adj[i][0],
                    piece_coordinates[1] + condition_adj[i][1],
                ];
    
                // Do conditional capture when conditions are met
                if fits_in_board(capture_coordinates) && fits_in_board(condition_coordinates) { // Check both the capture and condition coordinates fit in the board

                    // Check conditions
                    if get_board(condition_coordinates, last_turn_board) == 1 { // Piece in condition square must have moved last turn
                        if !friendly_piece(id, get_board(condition_coordinates, board)) { // Condition square must be occupied by an enemy piece
                            if get_board(condition_coordinates, turns_board) == condition_subj_moves { // Piece in condition square must have moved condition_subj_moves times
                                if piece_coordinates[1] == condition_self_y { // Piece performing the special capture must be at y coordinates condition_self_y
                                    if get_board(capture_coordinates, board) == 0 { // Square where the piece moves to must be empty

                                        // Set moves_board to 1 at the capture coordinates to indicate that the piece can move there
                                        moves_board = set_board(capture_coordinates, 1, moves_board); 

                                        // Set moves_board to -1 at the condition coordinates to indicate that the piece there should be captured
                                        moves_board = set_board(condition_coordinates, -1, moves_board); 
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        

        // If the piece is sliding set repeats to the maximum ammount it needs to be for the current board size
        // Repeats makes one mdir apply repeatedly until it reaches the end of the board
        let mut repeats: usize = 1;
        if slides {
            if BOARD_SIZE[0] > BOARD_SIZE[1] {
                repeats = BOARD_SIZE[0];
            }
            repeats = BOARD_SIZE[1];
        }
        
        // Generate standard moves
        for i in 0..mdir_no {
            let mut piece_coordinates_current = piece_coordinates;
            for j in 0..repeats {

                // Get the pieces coordinates after applying the x and y coordinates change from mdirs array
                let move_coordinates = [
                    piece_coordinates_current[0] + mdirs[i][0],
                    piece_coordinates_current[1] + mdirs[i][1],
                ];

                if fits_in_board(move_coordinates) { // Check move coordinates fit in the board
                    let move_coordinates_id =  get_board(move_coordinates, board);

                    if move_coordinates_id == 0 { // If the move_coordinates are empty they can be moved to
                        moves_board = set_board(move_coordinates, 1, moves_board);
                        piece_coordinates_current = move_coordinates;
                    } else if !friendly_piece(id, move_coordinates_id) && move_coordinates_id != 0 { // If the move_coordinates are an enemy they can be moved to
                        moves_board = set_board(move_coordinates, 1, moves_board);
                        piece_coordinates_current = move_coordinates;
                        break;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        moves_board
    }

    
    pub fn castle(
    piece_coordinates: [i8; 2],
    move_coordinates: [i8; 2],
    mut board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    turns_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    enemy_moves_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    pieces: [info::Piece; 6])
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {

        let id = get_board(piece_coordinates, board);
        
        // Check the piece being moved is a king, the king has moves 0 times, and the king is not in check
        if id == pieces[5].id && get_board(piece_coordinates, turns_board) == 0 && get_board(piece_coordinates, enemy_moves_board) == 0 {

            // King castle mdirs
            let king_mdir_repeats: usize = 2; // How many times to repeat king_mdirs to get to castle position
            let king_mdirs: [[i8; 2]; 2] = [
                [1, 0],
                [-1, 0],
            ];
            
            // Rook castle mdirs
            // Rook mdirs do not get repeated like the kings do
            let rook_mdirs: [[i8; 2]; 2] = [
                [-2, 0],
                [3, 0],
            ];

            // Where the rooks have to be inorder to perform a castle
            let rook_coordinates: [[i8; 2]; 2] = [
                [7, 0],
                [0, 0],
            ];
            println!("{:?}", rook_coordinates);

            // Repeat twice because there are 2 directions which a king can castle into
            for i in 0..2 {
                if get_board(rook_coordinates[i], board) == pieces[1].id && get_board(rook_coordinates[i], turns_board) == 0 { // Check the rook for this castle direction is in the correct position and has moved 0 times

                    let move_coordinates_rook = [
                        rook_coordinates[i][0] + rook_mdirs[i][0],
                        rook_coordinates[i][1] + rook_mdirs[i][1],
                    ];

                    let mut piece_coordinates_current = piece_coordinates;
                    for j in 0..king_mdir_repeats {

                        let move_coordinates_king = [
                            piece_coordinates_current[0] + king_mdirs[i][0],
                            piece_coordinates_current[1] + king_mdirs[i][1],
                        ];

                        // Ensures the king cannot into check or through an enemy sightline
                        if get_board(move_coordinates_king, board) != 0 || get_board(move_coordinates_king, enemy_moves_board) != 0 {
                            break;
                        } else if move_coordinates_king == move_coordinates && j > 0 { // A castle is valid when these conditions are met and the first if conditions are not met
                            board = set_board(move_coordinates_king, id, board); // Move king to castled positon
                            board = set_board(piece_coordinates, 0, board); // Remove king at original position

                            board = set_board(move_coordinates_rook, get_board(rook_coordinates[i], board), board); // Move rook to castle position
                            board = set_board(rook_coordinates[i], 0, board); // Remove rook at original position
                        }
                        piece_coordinates_current = move_coordinates_king;
                    }
                }
            }
            
        }
        board
    }
    

    #[cfg(test)]
    mod tests {
        use crate::fen;
        use super::*;

        #[test]
        fn gen_all_moves_test() { // Test generating all moves for black pieces on the board
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/2q3r1/p7/4P3/8/2N5/8/6P1");
            let expected = [[0, 0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 1, 1, 1, 1, 0, 1], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 0], [1, 1, 1, 1, 1, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 0]];

            for i in 0..1 {
                let moves_board = gen_all_moves(
                    board,
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    pieces,
                    false,
                );

                assert_eq!(moves_board, expected);
            }
        }

        #[test]
        fn sliding_test() { // Test friendly pieces blocking a sliding pieces path
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/2q3r1/p7/4P3/8/2N5/8/6P1");
            let expected = [[0, 0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 1, 1, 1, 1, 0, 1], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
            
            for i in 0..1{
                let moves_board = gen_moves(
                    [2, 6],
                    board,
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                    pieces,
                );

                assert_eq!(moves_board, expected);
            }
        }

        #[test]
        fn en_passant_test() { // Test en passant
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/8/7p/5pP1/8/8/8/8");

            let expected = [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, -1, 1, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0]];

            let moves_board = gen_moves(
                [6, 4],
                board,
                [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0],  [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                pieces,
            );

            assert_eq!(moves_board, expected);
        }

        #[test]
        fn left_castle_test() { // Test king trying to castle left with no obstacles
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/8/8/8/8/8/8/R3K2R");

            let result = castle(
                [4, 0],
                [2, 0],
                board,
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                pieces,
            );

            let expected = fen::decode("8/8/8/8/8/8/8/2KR3R");

            assert_eq!(result, expected);
        }

        #[test]
        fn block_castle_test() { // Test king trying to castle right through an obstacle
            let pieces = info::Piece::instantiate_all();
            let board = fen::decode("8/8/8/8/8/8/8/R3K2R");

            let result = castle(
                [4, 0],
                [6, 0],
                board,
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                pieces,
            );

            assert_eq!(result, board);
        }
    }
}