use crate::board::BOARD_SIZE;
use crate::piece::info::IDS;

// Z index corresponds to IDS index
// Heatmaps should only be used in the early stages of the game
pub const PIECE_HEATMAPS: [[[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]; IDS.len()] = [
    // Piece heatmaps appear rotated 90 degrees clockwise in the board arrays
    // Pawn heatmap
    [
        [0, 7, 0, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 0, 0],
        [0, 7, 3, 1, 0, 0, 0, 0],
        [0, 0, 1, 4, 1, 0, 0, 0],
        [0, 0, 1, 4, 1, 0, 0, 0],
        [0, 7, 3, 1, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 0, 0, 0, 0],
    ],

    // Rook heatmap
    [
        [0, 0, 3, 3, 3, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0, 0],
        [2, 0, 1, 1, 1, 0, 0, 0],
        [2, 0, 1, 1, 1, 0, 0, 0],
        [2, 0, 1, 1, 1, 0, 0, 0],
        [2, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 3, 3, 3, 0, 0, 0],
    ],

    // Knight heatmap
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],

    // Bishop heatmap
    [
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
    ],

    // Queen heatmap
    [
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
        [0, 1, 1, 1, 1, 0, 0, 0],
    ],

    // King heatmap
    [
        [4, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0],
    ],

    /*
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    */
];

pub mod minimax {
    use super::*;
    use std::collections::HashMap;

    use crate::board::turn::new_turn;
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct BranchValue {
        pub piece_coordinates: [i8; 2],
        pub move_coordinates: [i8; 2],
        pub value: i8,
        pub heatmap_value: i8,
    }

    // Struct assigned to board keys in the transposition table
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct TranspositionInfo {
        max: BranchValue,
        min: BranchValue,
        search_depth: usize,
        current_depth: usize,
    }

    impl BranchValue {
        pub fn new() -> Self {
            BranchValue {
                piece_coordinates: [0, 0],
                move_coordinates: [0, 0],
                value: 0,
                heatmap_value: 0,
            }
        }
    }

    pub fn best_move(
        master_team: bool,
        init_val: i8,
        search_depth: usize,
        current_depth: usize,
        parent_value: Option<i8>,
        bitstrings_board: &[[HashMap<i8, u64>; BOARD_SIZE[0]]; BOARD_SIZE[1]],
        transposition_table: &mut HashMap<u64, TranspositionInfo>,
        game_state: GameState)
        -> BranchValue {
        use crate::coordinates_from_usize;
        use crate::get_board;
        use crate::board::errors;
        use crate::gen_zobrist_board_hash;

        // Stop searching moves once the last branch is reached
        if current_depth == search_depth {
            return BranchValue {
                piece_coordinates: [0, 0],
                move_coordinates: [0, 0],
                value: init_val,
                heatmap_value: 0,
            };
        }

        // Generate moves
        let mut moves = order_moves(game_state);

        let board_hash = gen_zobrist_board_hash(game_state.whites_turn, game_state.board_info, &bitstrings_board);
        let transposition_value = transposition_table.get(&board_hash).copied();

        match transposition_value {
            Some(transposition_info) => {

                // If this position has allready been searched at the current depth return its results
                if transposition_info.search_depth >= search_depth && transposition_info.current_depth >= current_depth {
                    if master_team {
                        return transposition_info.max;
                    }
                    return transposition_info.min
                }
            },
            None => (),
        }

        let mut max = BranchValue::new();
        let mut min = max;

        let mut init_min_max = true;

        let mut min_max_val: Option<i8> = None;

        // Use best move from lower level search as the first move to search at this depth
        let mut deepening_val = max;
        let mut use_deepening_val = false;
        if current_depth == 0 && search_depth > 1 {
            deepening_val = best_move(master_team, init_val, search_depth - 1, current_depth, parent_value, bitstrings_board, transposition_table, game_state);
            use_deepening_val = true;
        }
        moves.rotate_right(1);
        moves[0] = Some(deepening_val);

        // Unwrap parent value
        let mut use_parent_value = false;
        let parent_value = match parent_value {
            Some(value) => {use_parent_value = true; value},
            None => 0,
        };

        for i in 0..moves.len() {
            let move_info = match moves[i] {
                Some(move_info) => move_info,
                None => break,
            };

            let piece_coordinates = move_info.piece_coordinates;
            let move_coordinates = move_info.move_coordinates;

            let mut move_error = false;
            let mut valid_move = true;

            // Get the material value of moving from piece_coordinates to move_coordinates
            let game_state_new = new_turn(piece_coordinates, move_coordinates, crate::piece::info::IDS[4], game_state); // The ai will only try to promote pawns to queens
            let mut move_val = match game_state_new {
                Ok(game_state) => game_state.points_delta,
                Err(error) => {
                    move_error = true;

                    // If the error was not a checkmate, or stalemate then the error was related to an invalid move
                    if error.error_code != errors::CHECKMATE_ERROR && error.error_code != errors::STALEMATE_ERROR {
                        valid_move = false;
                    } else { // If the error was a checkmate or stalemate return error.value
                        let mut error_val = error.value;

                        if !master_team {
                            error_val *= -1;
                        }
                        
                        return BranchValue {
                            piece_coordinates: piece_coordinates,
                            move_coordinates: move_coordinates,
                            value: error_val,
                            heatmap_value: 0,
                        };
                    }
                    

                    error.value
                },
            };

            // If the current branch is not the master team then it's move values are negative (because they negatively impact the master team)
            if !master_team {
                move_val *= -1
            }

            let branch_val = init_val + move_val;

            let piece_id = get_board(piece_coordinates, game_state.board_info.board).abs();
            let mut heatmap_val: i8 = 0;

            // Use heatmaps to encourage pieces to move to advantageous sqaurs
            // Only use heatmaps early in the game (when white + black points are < 18)
            if piece_id != 0 && game_state.white_points_info.points_total + game_state.black_points_info.points_total < 18 {
                let init_val = get_board(piece_coordinates, PIECE_HEATMAPS[usize::try_from(piece_id - 1).unwrap()]);
                let move_val = get_board(move_coordinates, PIECE_HEATMAPS[usize::try_from(piece_id - 1).unwrap()]);
                heatmap_val = move_val - init_val; // Get heatmap delta so worse positions aren't moved to from a good position
            }

            if !move_error { // Do not check child branches inscase of a move errorpoints_delta: i8,
                let child_min_max = best_move(!master_team, branch_val, search_depth, current_depth + 1, min_max_val, bitstrings_board, transposition_table, game_state_new.unwrap()); // Get min/max value of child branch
                
                // Update min and max with child value
                if init_min_max { // Initialize max and min value
                    max = BranchValue {
                        piece_coordinates: piece_coordinates,
                        move_coordinates: move_coordinates,
                        value: child_min_max.value,
                        heatmap_value: heatmap_val,
                    };

                    min = BranchValue {
                        piece_coordinates: piece_coordinates,
                        move_coordinates: move_coordinates,
                        value: child_min_max.value,
                        heatmap_value: heatmap_val,
                    };

                    if master_team {
                        min_max_val = Some(max.value);
                    } else {
                        min_max_val = Some(min.value);
                    }

                    init_min_max = false;
                } else if child_min_max.value > max.value || {child_min_max.value == max.value && heatmap_val > max.heatmap_value} { // Update max value
                    max = BranchValue {
                        piece_coordinates: piece_coordinates,
                        move_coordinates: move_coordinates,
                        value: child_min_max.value,
                        heatmap_value: heatmap_val,
                    };
                    if master_team {
                        min_max_val = Some(max.value);
                    }
                } else if child_min_max.value < min.value || {child_min_max.value == min.value && heatmap_val < min.heatmap_value} { // Update min value
                    min = BranchValue {
                        piece_coordinates: piece_coordinates,
                        move_coordinates: move_coordinates,
                        value: child_min_max.value,
                        heatmap_value: heatmap_val,
                    };
                    if !master_team {
                        min_max_val = Some(min.value);
                    }
                }

                // Alpha beta pruning
                if use_parent_value {
                    if master_team {
                        if max.value > parent_value {
                            break;
                        }
                    } else if min.value < parent_value {
                        break;
                    }
                }
            }
        }

        // Add board to transposition table
        transposition_table.insert(board_hash, TranspositionInfo {
            max: max,
            min: min,
            search_depth: search_depth,
            current_depth: current_depth,
        });

        if master_team { // Return max values for master team
            return max;
        }

        min
    }

    // Orders possible moves for a GameState into a vec
    fn order_moves(game_state: GameState) -> [Option<BranchValue>; {BOARD_SIZE[0] * BOARD_SIZE[1]}.pow(2)] {
        use crate::get_board;
        use crate::coordinates_from_usize;
        use crate::board::errors;
        use crate::piece::moves;

        let mut moves_array: [Option<BranchValue>; {BOARD_SIZE[0] * BOARD_SIZE[1]}.pow(2)] = [None; {BOARD_SIZE[0] * BOARD_SIZE[1]}.pow(2)];
        let mut moves: Vec<BranchValue> = Vec::new();
        
        
        for x_piece in 0..BOARD_SIZE[0] {
            for y_piece in 0..BOARD_SIZE[1] {
                let mut piece_coordinates = coordinates_from_usize([x_piece, y_piece]);
                let piece_id = get_board(piece_coordinates, game_state.board_info.board);

                // Don't get moves for pieces from the wrong team
                if game_state.whites_turn && piece_id < 0 {
                    continue;
                } else if !game_state.whites_turn && piece_id > 0 {
                    continue;
                }

                // Get the value of the piece at piece_coordinates
                let mut piece_value = 0;
                if piece_id != 0 {
                    piece_value = game_state.board_info.pieces[usize::try_from(piece_id.abs() - 1).unwrap()].value;
                }                

                for x_move in 0..BOARD_SIZE[0] {
                    for y_move in 0..BOARD_SIZE[1] {
                        let mut move_coordinates = coordinates_from_usize([x_move, y_move]);
                        let move_id = get_board(move_coordinates, game_state.board_info.board);

                        // Get the value of the piece at move_coordinates
                        let mut move_value = 0;
                        if move_id != 0 {
                            move_value = game_state.board_info.pieces[usize::try_from(move_id.abs() - 1).unwrap()].value;
                        }

                        let move_board = moves::gen_move_board(piece_coordinates, move_coordinates, crate::piece::info::IDS[4], game_state.board_info);
                        if move_board.board != game_state.board_info.board { // If the move board is different to the initial board then the move is valid
                            let enemy_moves_board = moves::gen_enemy_moves(game_state.whites_turn, move_board);
                            let moves_board = moves::gen_all_moves(game_state.whites_turn, None, move_board);

                            let mut move_points_change = move_value;

                            // Assume the enemy will try to trade if the square is not defended
                            if get_board(move_coordinates, enemy_moves_board) == 1 && get_board(move_coordinates, moves_board) == 0 {
                                move_points_change -= piece_value;
                            }

                            if piece_coordinates == [1, 0] && move_coordinates == [1, 6] {
                                //println!("{}", move_points_change);
                            }

                            // Add move to moves vec
                            moves.push(BranchValue {
                                piece_coordinates: piece_coordinates,
                                move_coordinates: move_coordinates,
                                value: move_points_change,
                                heatmap_value: 0,
                            });
                        }
                    }
                }
            }
        }

        // Sort moves and return
        moves.sort_by(|a, b| b.value.cmp(&a.value));
        
        // Add moves vec to moves array
        for i in 0..moves.len() {
            moves_array[i] = Some(moves[i]);
        }
        moves_array
    }



    #[cfg(test)]
    mod tests {
        use crate::fen;
        use crate::board::turn::PointsInfo;
        use crate::piece::moves::BoardInfo;
        use super::*;

        #[test]
        fn best_move_test1() {
            let game_state = GameState {
                white_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                black_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                points_delta: 0,

                board_info: BoardInfo {
                    board: fen::decode("8/8/8/8/8/r2r4/3R3n/8"),
                    turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            let mut transposition_table: HashMap<u64, TranspositionInfo> = HashMap::new();
            let bitstrings_board = crate::gen_bistrings_board();

            assert_eq!(best_move(true, 0, 3, 0, None, &bitstrings_board, &mut transposition_table, game_state).move_coordinates, [7, 1]);
        }

        #[test]
        fn best_move_test2() {
            let game_state = GameState {
                white_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                black_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                points_delta: 0,

                board_info: BoardInfo {
                    board: fen::decode("8/8/8/4p3/3b1p2/4P3/8/8"),
                    turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            let mut transposition_table: HashMap<u64, TranspositionInfo> = HashMap::new();
            let bitstrings_board = crate::gen_bistrings_board();

            assert_eq!(best_move(true, 0, 3, 0, None, &bitstrings_board, &mut transposition_table, game_state).move_coordinates, [3, 3]);
        }

        #[test]
        fn best_move_test3() {
            let game_state = GameState {
                white_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                black_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                points_delta: 0,

                board_info: BoardInfo {
                    board: fen::decode("k7/1p6/6r1/8/8/5B2/8/1Q6 w - - 0 1"),
                    turns_board: [[1i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            let mut transposition_table: HashMap<u64, TranspositionInfo> = HashMap::new();
            let bitstrings_board = crate::gen_bistrings_board();

            assert_eq!(best_move(true, 0, 3, 0, None, &bitstrings_board, &mut transposition_table, game_state).move_coordinates, [1, 6]);
        }

        #[test]
        fn order_moves_test() {
            let game_state = GameState {
                white_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                black_points_info: PointsInfo {
                    captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
                    captured_pieces_no: 0,
                    points_total: 0,
                    points_delta: 0,
                },

                points_delta: 0,

                board_info: BoardInfo {
                    board: fen::decode("2n5/7n/8/6R1/8/8/8/2Q3R1"),
                    turns_board: [[1i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            let result = order_moves(game_state);
            let best_move = BranchValue {
                piece_coordinates: [2, 0],
                move_coordinates: [2, 7],
                value: 3,
                heatmap_value: 0,
            };

            assert_eq!(result[0], Some(best_move));
        }
    }
}