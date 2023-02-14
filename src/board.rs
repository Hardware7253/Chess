// Board size is able to be changed, unit tests are likely to break
pub const BOARD_SIZE: [usize; 2] = [8, 8];
pub const MAX_SLIDES: usize = 8; // Maximum number of squares a piece should be able to move to, this is equal to the longest side of the board

// niighfg

pub mod errors {
    pub const CHECK_ERROR: i8 = 1;
    pub const CHECKMATE_ERROR: i8 = 2;
    pub const STALEMATE_ERROR: i8 = 3;
    pub const INVALID_MOVE_ERROR: i8 = 4;
    pub const WRONG_TEAM_ERROR: i8 = 5;

    // Returns an error message given an error code
    pub fn message(error_code: i8) -> Option<String> {
        let error_message: &str = match error_code {
            CHECK_ERROR => "Invalid move, king is in check",
            CHECKMATE_ERROR => "Checkmate",
            STALEMATE_ERROR => "Stalemate",
            INVALID_MOVE_ERROR => "Invalid move",
            WRONG_TEAM_ERROR => "Invalid move, please move a piece from the correct team",
            other => return None,
        };
        Some(String::from(error_message))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn message_test() {
            use crate::unwrap_def;

            let result = message(CHECKMATE_ERROR);

            let expected = String::from("Checkmate");
            assert_eq!(unwrap_def(result, String::from("Fail")), expected);
        }
    }
}

// Module containing functions related to making a new turn
pub mod turn {
    use super::*;
    use crate::piece::moves::BoardInfo;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct PointsInfo {
        pub captured_pieces: [i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}], // Array of piece ids that have been captured
        pub captured_pieces_no: i8, // Number of pieces that have been captured
        pub points_total: i8, // Total points
        pub points_delta: i8, // Last points change
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct GameState {
        white_points_info: PointsInfo,
        black_points_info: PointsInfo,
        board_info: BoardInfo,
        whites_turn: bool,
    }

    pub struct Error {
        white_win: Option<bool>,
        message: String,
    }

    // Points info stored information on captured pieces, and points of the team that it belongs to
    // Function looks at old and new BoardInfo to find captured pieces
    // Captured pieces added to captured pieces array and are used to calculate points total and points change
    fn update_points_info(
    board_info_old: BoardInfo,
    board_info_new: BoardInfo,
    mut points_info: PointsInfo)
    -> PointsInfo {
        use crate::unwrap_def;
        use crate::get_board;

        let pieces = board_info_old.pieces;

        // Get coordiantes of captured piece
        let capture_coordinates = board_info_new.capture_coordinates;
        let mut force_capture_coordinates = false;
        match capture_coordinates {
            Some(_) => force_capture_coordinates = true,
            None => force_capture_coordinates = false,
        };

        let mut captured_coordinates = board_info_new.last_turn_coordinates;
        if force_capture_coordinates {
            captured_coordinates = unwrap_def(capture_coordinates, [0i8; 2]); 
        }

        let captured_piece_id = get_board(captured_coordinates, board_info_old.board); // Get id of captured piece
        if captured_piece_id == 0 { // If captured piece id is 0 no pieces were captured
            return points_info;
        }

        let points_change = pieces[usize::try_from(captured_piece_id.abs() - 1).unwrap()].value; // Get value of captured piece

        // Update points_info struct
        points_info.captured_pieces[usize::try_from(points_info.captured_pieces_no).unwrap()] = captured_piece_id;
        points_info.captured_pieces_no = points_info.captured_pieces_no + 1;
        points_info.points_total = points_info.points_total + points_change;
        points_info.points_delta = points_change;

        points_info
    }

    pub fn new_turn(
    piece_coordinates: [i8; 2],
    move_coordinates: [i8; 2],
    game_state: GameState) -> Result<GameState, Error> {
        use crate::get_board;
        use crate::piece_white;
        use crate::piece::moves::gen_move_board;
        use crate::unwrap_def;
        use crate::flip_board;
        use crate::flip_coordinates;
        
        let board_info = game_state.board_info;

        // Return an error when a player tries to move a piece from the wrong team
        if piece_white(get_board(piece_coordinates, board_info.board)) != game_state.whites_turn {
            return Err(Error {
                white_win: None,
                message: unwrap_def(errors::message(errors::WRONG_TEAM_ERROR), String::from("Error")),
            });
        }

        let board_info_new = gen_move_board(piece_coordinates, move_coordinates, board_info);

        // Return error if board_info_new contains an error
        let error_message = errors::message(board_info_new.error_code);
        let error = match error_message {
            Some(_) => true,
            None => false,
        };
        if error {
            return Err(Error {
                white_win: None,
                message: unwrap_def(error_message, String::from("Error")),
            });
        }

        // At this point there was no error with the move
        // So get and return a new GameState
        // The new gamestate will be the state for the next turn

        let mut game_state_new = game_state;

        // Update points
        let mut points;
        if game_state.whites_turn {
            points = game_state.white_points_info;
        } else {
            points = game_state.black_points_info;
        }

        let points_new = update_points_info(board_info, board_info_new, points);

        if game_state.whites_turn {
            game_state_new.white_points_info = points_new;
        } else {
            game_state_new.black_points_info = points_new;
        }

        // It will be the opposite teams move after this so flip board_info
        game_state_new.board_info.board = flip_board(board_info_new.board);
        game_state_new.board_info.turns_board = flip_board(board_info_new.turns_board);
        game_state_new.board_info.last_turn_coordinates = flip_coordinates(board_info_new.last_turn_coordinates);

        // Invert whites_turn bool to set the next turn to be the opposite team
        game_state_new.whites_turn = !game_state.whites_turn;

        Ok(game_state_new)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::fen;
        use crate::piece::moves::BoardInfo;

        // update_points_info test -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
        #[test]
        fn update_points_info_test1() { // Test update_points_info when a regular capture takes place
            let board_info_old = BoardInfo {
                board: fen::decode("8/8/8/3b4/8/4N3/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0, 0],
                capture_coordinates: None,
                error_code: 0,
                pieces: crate::piece::info::Piece::instantiate_all(),
            };

            let board_info_new = BoardInfo {
                board: fen::decode("8/8/8/3N4/8/8/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [3, 4],
                capture_coordinates: None,
                error_code: 0,
                pieces: crate::piece::info::Piece::instantiate_all(),
            };

            let mut points_info = PointsInfo {
                captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                captured_pieces_no: 1,
                points_total: 4,
                points_delta: 0,
            };
            points_info.captured_pieces[0] = -1;

            let result = update_points_info(board_info_old, board_info_new, points_info);
            
            let mut expected = PointsInfo {
                captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                captured_pieces_no: 2,
                points_total: 7,
                points_delta: 3,
            };
            expected.captured_pieces[0] = -1;
            expected.captured_pieces[1] = -4;

            assert_eq!(result, expected);
            

        }

        #[test]
        fn update_points_info_test2() { // Test update_points_info when a conditional capture takes place
            let board_info_old = BoardInfo {
                board: fen::decode("8/8/8/4pP2/8/8/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [0, 0],
                capture_coordinates: None,
                error_code: 0,
                pieces: crate::piece::info::Piece::instantiate_all(),
            };

            let board_info_new = BoardInfo {
                board: fen::decode("8/8/4P3/8/8/8/8/8"),
                turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]],
                last_turn_coordinates: [4, 5],
                capture_coordinates: Some([4, 4]),
                error_code: 0,
                pieces: crate::piece::info::Piece::instantiate_all(),
            };

            let mut points_info = PointsInfo {
                captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                captured_pieces_no: 1,
                points_total: 4,
                points_delta: 0,
            };
            points_info.captured_pieces[0] = -1;

            let result = update_points_info(board_info_old, board_info_new, points_info);
            
            let mut expected = PointsInfo {
                captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                captured_pieces_no: 2,
                points_total: 5,
                points_delta: 1,
            };
            expected.captured_pieces[0] = -1;
            expected.captured_pieces[1] = -1;

            assert_eq!(result, expected);
            

        }
        // update_points_info test -----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
    }
}