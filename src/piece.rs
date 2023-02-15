use crate::board::BOARD_SIZE;

// Module containing piece information such as IDS and moves
pub mod info {
    // Piece ids, order must not change, ids can
    // Order                  P  R  N  B  Q  K
    pub const IDS: [i8; 6] = [1, 2, 3, 4, 5, 6];

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Piece {
        pub id_fen: char,
        pub id: i8,   

        pub value: i8,

        pub sliding: bool,

        // Used to restrict the amount of times a piece can slide (E.g. can only slide twice). Only takes affect for a pieces first turn (moves = 0)
        pub slide_no: Option<usize>, 

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

                sliding: true,
                slide_no: Some(2),

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
                slide_no: None,

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
                slide_no: None,

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
                slide_no: None,

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
                slide_no: None,

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
                slide_no: None,

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

// Module containg functions related to moving piece(s)
pub mod moves {
    use super::*;

    // Bring library functions into scope
    use crate::unwrap_def;
    use crate::fits_in_board;
    use crate::get_board;
    use crate::set_board;
    use crate::friendly_piece;
    use crate::piece_white;
    use crate::move_board_value;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Moves {
        pub moves_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]], // Stores information of where a piece can move, and what squares a piece can put in check
        pub capture_coordinates: Option<[i8; 2]>, // Used when a piece is captured but it's square is not taken by the piece capturing (en passant)
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct CheckType {
        // For a check mate (check) and (mate) have to be true
        // For a stale mate just (mate) should be true
        // For a check just (check) should be true
        pub check: bool, // Where an enemy can capture the king
        pub mate: bool, // Where the king cannot move to any of it's surrounding squares

    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct BoardInfo {
        pub board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]], // Game board, stores piece ids in the positions they are on the board.
        pub turns_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]], // Turns board, values correspond to a piece at the same coordinates on the game board. Values represent how many times that piece has moved from its starting position.
        pub last_turn_coordinates: [i8; 2], // Last turn coordinates, coordinates of the piece that moved last turn.
        pub capture_coordinates: Option<[i8; 2]>, // Coordinates of piece that was captured (if any)
        pub error_code: i8,
        pub pieces: [info::Piece; 6], // Array stores piece structs, structs contain infromation such as piece ids, movement directions, and movement types.
    }

    // Generates all possible moves given a single piece, cannot generate moves for an enemy team because the pawns will move backwards
    fn gen_moves(mut piece_coordinates: [i8; 2],
    mut moves_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]], // Allows a custom starting moves_board to be set, this allows moves to be added to a pre-existing moves_board
    board_info: BoardInfo)
    -> Moves {
        use crate::board::MAX_SLIDES;
        
        let board = board_info.board;
        let turns_board = board_info.turns_board;
        let last_turn_coordinates = board_info.last_turn_coordinates;
        let pieces = board_info.pieces;

        // Get piece id
        let id = get_board(piece_coordinates, board);
        if id == 0 {
            return Moves {
                moves_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                capture_coordinates: None,
            } 
        }

        // If the piece has a negative id change it to positive so it can be used to index pieces array
        let mut pieces_index = id.abs() - 1;

        // mdirs information (movement directions)
        let mdirs = pieces[usize::try_from(pieces_index).unwrap()].mdirs;
        let mut mdir_no = pieces[usize::try_from(pieces_index).unwrap()].mdir_no;
        let mut slides = pieces[usize::try_from(pieces_index).unwrap()].sliding;
        
        // Unwrap slide_no Option<T>
        let slide_no = pieces[usize::try_from(pieces_index).unwrap()].slide_no;
        let mut slide_no = unwrap_def(slide_no, MAX_SLIDES);
        
        if get_board(piece_coordinates, turns_board) != 0 && slide_no != MAX_SLIDES {

            // Piece must have 0 turns to make use of a custom slide number
            // If it doesn't slding is disabled
            slides = false;
        }

        if !slides {
            slide_no = 1;
        }

        println!("{}", slide_no);
    
        
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

        let mut captured_coordinates: Option<[i8; 2]> = None;

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
                    if condition_coordinates == last_turn_coordinates { // Piece in condition square must have moved last turn
                        let condition_id = get_board(condition_coordinates, board);
                        if !friendly_piece(id, condition_id) && id.abs() == condition_id.abs() { // Condition square must be occupied by an enemy piece, and the piece capturing and piece being captured must be of the same type
                            if get_board(condition_coordinates, turns_board) == condition_subj_moves { // Piece in condition square must have moved condition_subj_moves times
                                if piece_coordinates[1] == condition_self_y { // Piece performing the special capture must be at y coordinates condition_self_y
                                    if get_board(capture_coordinates, board) == 0 { // Square where the piece moves to must be empty

                                        // Set moves_board to 1 at the capture coordinates to indicate that the piece can move there
                                        moves_board = set_board(capture_coordinates, 1, moves_board); 

                                        captured_coordinates = Some(condition_coordinates);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Generate regular moves

        // Get id of enemy king as no_block_id
        // Because enemy kings do not block piece sightlines
        let mut no_block_id = info::IDS[5];
        if slide_no != MAX_SLIDES { // If slide_no != MAX_SLIDES then the piece is not a sliding piece
            no_block_id = 0 // Non sliding pieces do not use no_block_id
        } else if id > 0 {
            no_block_id = no_block_id * -1;
        }

        for i in 0..mdir_no {
            let mut piece_coordinates_current = piece_coordinates;
            for j in 0..slide_no {

                // Get the pieces coordinates after applying the x and y coordinates change from mdirs array
                let move_coordinates = [
                    piece_coordinates_current[0] + mdirs[i][0],
                    piece_coordinates_current[1] + mdirs[i][1],
                ];

                if fits_in_board(move_coordinates) { // Check move coordinates fit in the board
                    let move_coordinates_id =  get_board(move_coordinates, board);

                    // Default move val is 1
                    let mut move_val = 1;
                    if special_capture {
                        move_val = 2; // 2 When the move should not be seen as a potential capture (e.g. can't put the king in check)
                    }

                    if move_coordinates_id == 0 || move_coordinates_id == no_block_id { // If the move_coordinates are empty they can be moved to
                        moves_board = set_board(move_coordinates, move_val, moves_board);
                        piece_coordinates_current = move_coordinates;
                    } else if !friendly_piece(id, move_coordinates_id) && move_coordinates_id != 0 && !special_capture { // If the move_coordinates are an enemy they can be moved to, special captures cannot capture this way, they have to use their special capture
                        moves_board = set_board(move_coordinates, move_val, moves_board);
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

        // Return moves board and captured coordinates
        Moves {
            moves_board: moves_board,
            capture_coordinates: captured_coordinates,
        }
    }

    // Generates all possible moves for a type of piece (white or black)
    fn gen_all_moves(
    gen_all_white: bool, // When true generates all white moves, generates black mvoes when false
    ignore_id: Option<i8>,
    board_info: BoardInfo)
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {        
        let mut moves_board = [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]];
        let ignore_id = unwrap_def(ignore_id, 0);
        for x in 0..BOARD_SIZE[0] {
            for y in 0..BOARD_SIZE[1] {
                let id = board_info.board[x][y]; // Get id of piece at board coordinates (x, y)

                if piece_white(id) == gen_all_white && id != 0 && id != ignore_id { // Check piece type matches piece type defined in gen_all_white
                    let piece_coordinates = [x.try_into().unwrap(), y.try_into().unwrap()];

                    moves_board = gen_moves(piece_coordinates, moves_board, board_info).moves_board;
                }
            }
        }
        moves_board
    }

    // Generates all moves of the enemy team
    // Flip boards to enemy perspective to fix the problem where enemy pawns move backwards
    fn gen_enemy_moves(
    caller_white: bool,
    board_info: BoardInfo)
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {
        use crate::flip_board;
        use crate::flip_coordinates;
        
        // Flip board_info to get enemy perspective
        let board_info = BoardInfo {
            board: flip_board(board_info.board),
            turns_board: flip_board(board_info.turns_board),
            last_turn_coordinates: flip_coordinates(board_info.last_turn_coordinates),
            capture_coordinates: None,
            error_code: 0,
            pieces: board_info.pieces,
        };

        let enemy_moves = gen_all_moves(!caller_white, None, board_info);
        flip_board(enemy_moves) // Flip enemy moves again to get back to perspective of the caller team
    }
    
    // Gen enemy moves, except the enemy paths are blocked by friendly pieces paths
    fn gen_enemy_moves_blocked(caller_white: bool, ignore_id: Option<i8>, board_info: BoardInfo) -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {

        // Add friendly moves blocking paths to current board
        let mut friendly_moves = gen_all_moves(caller_white, ignore_id, board_info);
        if !caller_white {
            friendly_moves = crate::replace_in_board(1, -1, friendly_moves);
        }
        let board_with_moves = crate::combine_boards(board_info.board, friendly_moves, 0);

        // Gen enemy moves with modified board
        let board_info = BoardInfo {
            board: board_with_moves,
            turns_board: board_info.turns_board,
            last_turn_coordinates: board_info.last_turn_coordinates,
            capture_coordinates: board_info.capture_coordinates,
            error_code: 0,
            pieces: board_info.pieces,
        };

        gen_enemy_moves(caller_white, board_info)
    }
    
    // Given original piece coordinates and move coordinates this function checks if the move coordinates are valid for a castle
    // If a castle is possible a new board is returned where the king and rook pieces have castled, otherwise the original board is returned
    fn castle(
    piece_coordinates: [i8; 2],
    move_coordinates: [i8; 2],
    enemy_moves_board: [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
    board_info: BoardInfo)
    -> [[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]] {

        let mut board = board_info.board;
        let turns_board = board_info.turns_board;
        let pieces = board_info.pieces;

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
                            board = move_board_value(piece_coordinates, move_coordinates_king, 0, board); // Move king to castled position
                            board = move_board_value(rook_coordinates[i], move_coordinates_rook, 0, board); // Move rook to castled position
                        }
                        piece_coordinates_current = move_coordinates_king;
                    }
                }
            }
            
        }
        board
    }

    // Get check status of a king from a given team (white)
    pub fn get_check_state(white: bool, get_mate: bool, board_info: BoardInfo) -> CheckType {
        // Get king_id for team specified by (white)
        let mut king_id = info::IDS[5];
        if !white {
            king_id = king_id * -1;
        };

        // Get king coordinates
        // No error handling for when no king is found, since it should not occur in a normal chess game.
        let king_coordinates = crate::find_id_in_board(king_id, board_info.board);
        let king_coordinates = unwrap_def(king_coordinates, [0, 0]);

        let enemy_moves_board = gen_enemy_moves(white, board_info);

        // Check for king check
        let mut check = false;
        if get_board(king_coordinates, enemy_moves_board) == 1 {
            check = true;
        }

        let mut mate = false;
        if get_mate {
            let enemy_moves_board = gen_enemy_moves_blocked(white, Some(king_id), board_info);

            let king_mdirs = board_info.pieces[usize::try_from(king_id.abs() - 1).unwrap()].mdirs;
            let king_mdir_no = board_info.pieces[usize::try_from(king_id.abs() - 1).unwrap()].mdir_no;

            // Check for king mate
            // If every square the king can move to would put it in check, then the mate condition is true
            mate = true;
            for i in 0..king_mdir_no {
                // Get king move coordinates from current position
                let move_coordiantes = [
                    king_coordinates[0] + king_mdirs[i][0],
                    king_coordinates[1] + king_mdirs[i][1],
                ];

                if fits_in_board(move_coordiantes) { // Check move is valid
                    if !friendly_piece(king_id, get_board(move_coordiantes, board_info.board)) { // Check move is valid
                        if get_board(move_coordiantes, enemy_moves_board) != 1 {
                            mate = false;
                        }
                    }
                }
            }
        }
        
        CheckType {
            check: check,
            mate: mate,
        }
    }

    // Given original piece coordinates and move coordinates returns a new board where the piece is moved to the new coordinates
    // This only happens when the move is valid
    pub fn gen_move_board(
    piece_coordinates: [i8; 2],
    move_coordinates: [i8; 2],
    mut board_info: BoardInfo)
    -> BoardInfo {
        use crate::board::errors;

        let board = board_info.board;
        let turns_board = board_info.turns_board;
        let last_turn_coordinates = board_info.last_turn_coordinates;
        let pieces = board_info.pieces;

        let piece_white = piece_white(get_board(piece_coordinates, board));
        let id = get_board(piece_coordinates, board);

        let mut move_valid = false;

        let mut error = 0;

        // Castle
        let mut castle_board = board;
        if id == pieces[5].id {
            let enemy_moves = gen_enemy_moves(piece_white, board_info);
            castle_board = castle(piece_coordinates, move_coordinates, enemy_moves, board_info);
        }
        if castle_board != board {
            move_valid = true;
        }

        // Board info post move
        let mut board_info_pm = board_info;
        board_info_pm.error_code = 0;
        board_info_pm.board = castle_board;
        board_info_pm.capture_coordinates = None;

        println!("{:?}", piece_coordinates);

        // Generate possible moves for the piece at piece_coordinate
        let moves = gen_moves(
            piece_coordinates,
            [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
            board_info,
        );
        let possible_moves = moves.moves_board;
        let capture_coordinates = moves.capture_coordinates;
        
        println!("{}", get_board(move_coordinates, possible_moves));

        // If possible_moves at move_coordinates != 0 then the piece can move there
        if get_board(move_coordinates, possible_moves) != 0 && !move_valid {

            // Get board where the piece at piece_coordinates is moved to move_coordinates
            let mut post_move_board = move_board_value(piece_coordinates, move_coordinates, 0, board);

            // Check if there are coordinates which have to be captured
            let mut force_capture_coordinates = false;
            match capture_coordinates {
                Some(_) => force_capture_coordinates = true,
                None => force_capture_coordinates = false,
            };

            // Capture force_capture_coordinates
            if force_capture_coordinates {
                let capture_coordinates_unwrap = unwrap_def(capture_coordinates, [0i8; 2]);
                post_move_board = set_board(capture_coordinates_unwrap, 0, post_move_board); // Remove piece at capture_coordinates
                board_info_pm.turns_board = set_board(capture_coordinates_unwrap, 0, board_info_pm.turns_board); // Set turns at capture_coordinates to 0
                board_info_pm.capture_coordinates = capture_coordinates; // Set capture coordinates
            }

            board_info_pm.board = post_move_board;
            let enemy_moves = gen_enemy_moves(piece_white, board_info_pm);
            
            // If the king isn't in check after the move, then the move is valid
            let king_check_state = get_check_state(piece_white, false, board_info_pm);
            if !king_check_state.check {
                move_valid = true;
            } else {
                error = errors::CHECK_ERROR;
            }
        } else {
            println!("True");
            error = errors::INVALID_MOVE_ERROR;
        }

        if move_valid {
            // Move and increment piece turns
            board_info_pm.turns_board = move_board_value(piece_coordinates, move_coordinates, 0, board_info_pm.turns_board);
            let turns = get_board(move_coordinates, board_info_pm.turns_board);
            let turns = turns + 1;
            
            board_info_pm.turns_board = set_board(move_coordinates, turns, board_info_pm.turns_board);

            // Set last moved piece
            board_info_pm.last_turn_coordinates = move_coordinates;
            
            return board_info_pm;
        }

        // Return board with error code
        board_info.error_code = error;
        board_info
    }
    

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::fen;

        // gen_moves tests ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        #[test]
        fn queen_sliding_test() { // Test generating queen moves, where some directions are blocked by enemy or friendly pieces
            let board_info = BoardInfo {
                board: fen::decode("8/2P5/8/P3p3/8/2Q5/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_moves(
                [2, 2],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                board_info,
            );

            let expected = Moves {
                moves_board: [[1, 0, 1, 0, 0, 0, 0, 0], [0, 1, 1, 1, 0, 0, 0, 0], [1, 1, 0, 1, 1, 1, 0, 0], [0, 1, 1, 1, 0, 0, 0, 0], [1, 0, 1, 0, 1, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0]],
                capture_coordinates: None,
            };
            assert_eq!(moves_board, expected);
        }

        #[test]
        fn en_passant_test() { // Test en passant
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/5pP1/8/8/8/8"),
                turns_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0],  [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                last_turn_coordinates: [5, 4],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_moves(
                [6, 4],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                board_info,
            );

            let expected = Moves {
                moves_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], [0, 0, 0, 0, 0, 2, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                capture_coordinates: Some([5, 4]),
            };
            assert_eq!(moves_board, expected);
        }

        #[test]
        fn double_move_test() { // Test pawn double move
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/8/8/8/1P6/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_moves(
                [1, 1],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                board_info,
            );

            let expected = Moves {
                moves_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 2, 2, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                capture_coordinates: None,
            };
            assert_eq!(moves_board, expected);
        }

        #[test]
        fn special_capture_test() { // Test pawn special capture direction
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/8/8/1pp5/2P5/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_moves(
                [2, 1],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                board_info,
            );

            let expected = Moves {
                moves_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                capture_coordinates: None,
            };
            assert_eq!(moves_board, expected);
        }
        // gen_moves tests ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

        #[test]
        fn gen_all_moves_test() { // Test generating all moves for white pieces on a board
            let board_info = BoardInfo {
                board: fen::decode("8/3b3r/5p2/b1p1p3/3p4/8/2Q2P2/R7"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_all_moves(
                true,
                None,
                board_info,
            );

            let expected = [[0, 1, 1, 1, 1, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [1, 0, 1, 1, 1, 0, 0, 0], [1, 1, 1, 0, 0, 0, 0, 0], [1, 1, 0, 1, 0, 0, 0, 0], [1, 0, 2, 2, 1, 0, 0, 0], [1, 0, 0, 0, 0, 1, 0, 0], [1, 0, 0, 0, 0, 0, 1, 0]];
            assert_eq!(moves_board, expected);
        }

        #[test]
        fn gen_enemy_moves_test() { // Test generating all enemy moves
            let board_info = BoardInfo {
                board: fen::decode("8/p1q3r1/8/4P3/8/2N5/8/6P1"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_enemy_moves(
                true,
                board_info,
            );

            let expected = [[0, 0, 0, 0, 2, 2, 0, 0], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 1, 1, 1, 1, 0, 1], [0, 0, 0, 0, 0, 1, 1, 1], [0, 0, 0, 0, 1, 0, 1, 0], [0, 0, 0, 0, 0, 0, 1, 0], [1, 1, 1, 1, 1, 1, 0, 1], [0, 0, 0, 0, 0, 0, 1, 0]];
            assert_eq!(moves_board, expected);
        }

        #[test]
        fn gen_enemy_moves_blocked_test() {
            let board_info = BoardInfo {
                board: fen::decode("8/4Q3/8/8/8/8/8/r7"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let moves_board = gen_enemy_moves_blocked(
                true,
                None,
                board_info,
            );

            let expected = [[0, 1, 1, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]];
            assert_eq!(moves_board, expected);
        }

        // castle tests ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        #[test]
        fn left_castle_test() { // Test king trying to castle left with no obstacles
            let board = fen::decode("8/8/8/8/8/8/8/R3K2R");
            let board_info = BoardInfo {
                board: board,
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = castle(
                [4, 0],
                [2, 0],
                [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                board_info,
            );

            let expected = fen::decode("8/8/8/8/8/8/8/2KR3R");

            assert_eq!(result, expected);
        }

        #[test]
        fn block_castle_test() { // Test king trying to castle through an obstacle
            let board = fen::decode("8/8/8/8/8/8/8/R3K2R");
            let board_info = BoardInfo {
                board: board,
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = castle(
                [4, 0],
                [6, 0],
                [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                board_info,
            );

            assert_eq!(result, board);
        }
        // castle tests ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        
        // get_check_state tests ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        #[test]
        fn get_check_state_test1() { // Test check mate
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/8/8/8/5PPP/1r4K1"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = get_check_state(true, true, board_info);

            let expected = CheckType {
                check: true,
                mate: true,
            };

            assert_eq!(result, expected);
        }

        #[test]
        fn get_check_state_test2() { // Test stale mate
            let board_info = BoardInfo {
                board: fen::decode("k7/2Q5/8/8/8/8/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = get_check_state(false, true, board_info);

            let expected = CheckType {
                check: false,
                mate: true,
            };

            assert_eq!(result, expected);
        }

        #[test]
        fn get_check_state_test3() { // Test check mate being blocked by friendly rook
            let board_info = BoardInfo {
                board: fen::decode("4R3/8/8/8/8/8/5PPP/r5K1"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = get_check_state(true, true, board_info);

            let expected = CheckType {
                check: true,
                mate: false,
            };

            assert_eq!(result, expected);
        }
        // get_check_state tests ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

        // gen_move_board tests (testing quite a few unique scenarios and edge cases)---------------------------------------------------------------------------------------------------------------------------------------
        #[test]
        fn gen_move_board_test1() { // Test an invalid move (blocked by check)
            let board_info = BoardInfo {
                board: fen::decode("8/8/6p1/3b4/8/8/6K1/1Q6"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 1,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [1, 0],
                [6, 5],
                board_info,
            );

            // The result is the original board because the king is in check
            // Moving the queen to g6 doesn't prevent check so it not a valid move
            assert_eq!(result, board_info);
        }

        #[test]
        fn gen_move_board_test2() { // Test a valid move
            let board_info = BoardInfo {
                board: fen::decode("8/8/6p1/8/8/8/6K1/1Q6"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [1, 0],
                [6, 5],
                board_info,
            );

            let expected = fen::decode("8/8/6Q1/8/8/8/6K1/8");
            assert_eq!(result.board, expected);
        }

        #[test]
        fn gen_move_board_test3() { // Test an invalid move (piece trying to move that puts the king in check)
            let board_info = BoardInfo {
                board: fen::decode("3RK3/B6B/2P1PN1Q/P2P4/2p3p1/6q1/p2n3p/1rk2r1b"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 1,
                pieces: info::Piece::instantiate_all(),
            };

            for i in 0..1 {
                let result = gen_move_board(
                    [3, 1],
                    [1, 2],
                    board_info,
                );
    
                assert_eq!(result, board_info);
            }
        }

        #[test]
        fn gen_move_board_test4() { // Test completely invalid move where a pawn tries to move to the other side of the board
            let board_info = BoardInfo {
                board: fen::decode("3RK3/B6B/2P1PN1Q/P2P4/2p3p1/6q1/p2n3p/1rk2r1b"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 4,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [0, 1],
                [7, 7],
                board_info,
            );

            assert_eq!(result, board_info);
        }

        #[test]
        fn gen_move_board_test5() { // Test king trying to move into check (where an enemy pawn puts the king in check)
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/8/8/8/1p6/1K6"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 1,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [1, 0],
                [0, 0],
                board_info,
            );

            assert_eq!(result, board_info);
        }

        #[test]
        fn gen_move_board_test6() { // Test pawn trying to capture king by moving forwards instead of diagonal
            let board_info = BoardInfo {
                board: fen::decode("6K1/6p1/8/8/8/8/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0i8; 2],
                capture_coordinates: None,
                error_code: 4,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [6, 6],
                [6, 7],
                board_info,
            );

            assert_eq!(result, board_info);
        }

        #[test]
        fn gen_move_board_test7() { // Test king castle
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/8/8/8/8/R3K2R"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0, 0],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [4, 0],
                [6, 0],
                board_info,
            );

            let expected =  BoardInfo {
                board: fen::decode("8/8/8/8/8/8/8/R4RK1"),
                turns_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                last_turn_coordinates: [6, 0],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            assert_eq!(result, expected);
        }

        #[test]
        fn gen_move_board_test8() { // Test turns_board and last_turns_coordinates being updated in gen_move_board (with en passant)
            let board_info = BoardInfo {
                board: fen::decode("8/8/8/4pP2/8/8/8/8"),
                turns_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 3, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                last_turn_coordinates: [4, 4],
                capture_coordinates: None,
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            let result = gen_move_board(
                [5, 4],
                [4, 5],
                board_info,
            );

            let expected =  BoardInfo {
                board: fen::decode("8/8/4P3/8/8/8/8/8"),
                turns_board: [[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 4, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0]],
                last_turn_coordinates: [4, 5],
                capture_coordinates: Some([4, 4]),
                error_code: 0,
                pieces: info::Piece::instantiate_all(),
            };

            assert_eq!(result, expected);
        }
        // gen_move_board tests --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    }
}