pub mod minimax {
    use crate::board::turn::new_turn;
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct BranchValue {
        pub piece_coordinates: [i8; 2],
        pub move_coordinates: [i8; 2],
        pub value: i8,
    }

    impl BranchValue {
        fn new() -> Self {
            BranchValue {
                piece_coordinates: [0, 0],
                move_coordinates: [0, 0],
                value: 0,
            }
        }
    }

    pub fn best_move(master_team: bool, init_val: i8, search_depth: usize, current_depth: usize, parent_value: Option<i8>, game_state: GameState) -> BranchValue {
        use crate::coordinates_from_usize;
        use crate::board::errors;

        // Stop searching moves once the last branch is reached
        if current_depth == search_depth {
            return BranchValue {
                piece_coordinates: [0, 0],
                move_coordinates: [0, 0],
                value: init_val,
            };
        }

        let mut max = BranchValue::new();
        let mut min = max;

        let mut init_min_max = true;

        let mut min_max_val: Option<i8> = None;

        
        // Initialize max and min value with best move using search depth - 1 (iterative deepening)
        let mut deepening_val = max;
        let mut use_deepening_val = false;
        if current_depth == 0 && search_depth > 1 {
            use_deepening_val = true;
            deepening_val = best_move(master_team, init_val, search_depth - 1, current_depth, parent_value, game_state);

            let mut valid_move = true;
            let game_state_new = new_turn(deepening_val.piece_coordinates, deepening_val.move_coordinates, crate::piece::info::IDS[4], game_state); // The ai will only try to promote pawns to queens
            let move_val = match game_state_new {
                Ok(game_state) => game_state.points_delta,
                Err(error) => {
                    if error.error_code != errors::CHECKMATE_ERROR || error.error_code != errors::STALEMATE_ERROR {
                        valid_move = false;
                    }

                    error.value
                },
            };

            if valid_move {
                let child_min_max = best_move(!master_team, move_val, search_depth, current_depth + 1, min_max_val, game_state_new.unwrap());
                max = BranchValue {
                    piece_coordinates: deepening_val.piece_coordinates,
                    move_coordinates: deepening_val.move_coordinates,
                    value: child_min_max.value,
                };

                min = BranchValue {
                    piece_coordinates: deepening_val.piece_coordinates,
                    move_coordinates: deepening_val.move_coordinates,
                    value: child_min_max.value,
                };

                if master_team {
                    min_max_val = Some(max.value);
                } else {
                    min_max_val = Some(min.value);
                }

                init_min_max = false;
            }
        }

        
        
        // Unwrap parent value
        let mut use_parent_value = false;
        let parent_value = match parent_value {
            Some(value) => {use_parent_value = true; value},
            None => 0,
        };

        'master: for x_piece in 0..BOARD_SIZE[0] {
            for y_piece in 0..BOARD_SIZE[1] {
                let mut piece_coordinates = coordinates_from_usize([x_piece, y_piece]);

                for x_move in 0..BOARD_SIZE[0] {
                    for y_move in 0..BOARD_SIZE[1] {
                        let mut move_coordinates = coordinates_from_usize([x_move, y_move]);
                        
                        // Skip piece and move coordinates that are the same as the deepening value
                        // Because deepening value initialized min and max itt does not need to be run again
                        if use_deepening_val {
                            if piece_coordinates == deepening_val.piece_coordinates && move_coordinates == deepening_val.move_coordinates {
                                break;
                            }
                        }

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

                        if !move_error { // Do not check child branches inscase of a move error
                            let child_min_max = best_move(!master_team, branch_val, search_depth, current_depth + 1, min_max_val, game_state_new.unwrap()); // Get min/max value of child branch
                            
                            // Update min and max with child value
                            if init_min_max { // Initialize max and min value
                                max = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: child_min_max.value,
                                };

                                min = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: child_min_max.value,
                                };

                                if master_team {
                                    min_max_val = Some(max.value);
                                } else {
                                    min_max_val = Some(min.value);
                                }

                                init_min_max = false;
                            } else if child_min_max.value > max.value { // Update max value
                                max = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: child_min_max.value,
                                };
                                if master_team {
                                    min_max_val = Some(max.value);
                                }
                            } else if child_min_max.value < min.value  { // Update min value
                                min = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: child_min_max.value,
                                };
                                if !master_team {
                                    min_max_val = Some(min.value);
                                }
                            }

                            // Alpha beta pruning
                            if use_parent_value {
                                if master_team {
                                    if max.value > parent_value {
                                        break 'master;
                                    }
                                } else if min.value < parent_value {
                                    break 'master;
                                }
                            }
                        }
                    }
                }
            }
        }

        if master_team { // Return max values for master team
            return max;
        }

        min
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

            assert_eq!(best_move(true, 0, 3, 0, None, game_state).move_coordinates, [7, 1]);
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

            assert_eq!(best_move(true, 0, 3, 0, None, game_state).move_coordinates, [3, 3]);
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

            assert_eq!(best_move(true, 0, 3, 0, None, game_state).move_coordinates, [1, 6]);
        }
    }
}