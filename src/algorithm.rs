pub mod minimax {
    use crate::board::turn::new_turn;
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

    // Notes
    // The board k7/1Q6/6r1/8/8/5B2/8/8 is not checkmate because the rook
    // is considered to be blocking the bishop.

    // To remedy this get check state has to be able to find the piece that
    // is putting the king in check, and only have sliding pieces block that
    // pieces path. Because in the example above the rook is blocking the friendly
    // bishops sightline. Which is making the function think there is no mate.

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct BranchValue {
        piece_coordinates: [i8; 2],
        move_coordinates: [i8; 2],
        value: i8,
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

    fn best_move(master_team: bool, init_val: i8, search_depth: usize, current_depth: usize, /*parent_value: Option<i8>,*/ game_state: GameState) -> BranchValue {
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
        let mut min = BranchValue::new();

        let mut max_branch = BranchValue::new();
        let mut min_branch = BranchValue::new();

        let mut init_min_max = true;
        let mut init_min_max_branch = true;

        let mut min_max_val = None;
        let mut min_max_val_branch: Option<i8> = None;

        for x_piece in 0..BOARD_SIZE[0] {
            for y_piece in 0..BOARD_SIZE[1] {
                let piece_coordinates = coordinates_from_usize([x_piece, y_piece]);

                for x_move in 0..BOARD_SIZE[0] {
                    for y_move in 0..BOARD_SIZE[1] {
                        let move_coordinates = coordinates_from_usize([x_move, y_move]);

                        let mut move_error = false;
                        let mut valid_move = true;

                        if piece_coordinates == [1, 0] && move_coordinates == [1, 6] && current_depth == 0 {
                            //println!("{:?}", new_turn(piece_coordinates, move_coordinates, game_state));
                        }

                        // Get the material value of moving from piece_coordinates to move_coordinates
                        let game_state_new = new_turn(piece_coordinates, move_coordinates, game_state);
                        let mut move_val = match game_state_new {
                            Ok(game_state) => game_state.points_delta,
                            Err(error) => {
                                move_error = true;

                                // If the error was not a checkmate, or stalemate then the error was related to an invalid move
                                if error.error_code != errors::CHECKMATE_ERROR && error.error_code != errors::STALEMATE_ERROR {
                                    valid_move = false;
                                } else {
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

                        let mut branch_val = init_val + move_val;

                        // Set branch val to move val incase of checkmate or stalemate
                        if move_error && valid_move {
                            branch_val = move_val;
                        }

                        /*
                        if branch_val > 50 {
                            println!("{}", branch_val);
                        }
                        */

                        if !move_error { // Do not check child branches inscase of a move error
                            let child_min_max = best_move(!master_team, branch_val, search_depth, current_depth + 1, game_state_new.unwrap()); // Get min/max value of child branch
                            
                            // Update min and max for child value
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

                        } else if valid_move { // Only update branch min max for valid moves (so no invalid moves are returned)

                            // Update min and max for branch value
                            if init_min_max_branch { // Initialize max and min value
                                max_branch = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: branch_val,
                                };

                                min_branch = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: branch_val,
                                };

                                init_min_max_branch = false;
                            } else if branch_val > max_branch.value { // Update max value
                                max_branch = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: branch_val,
                                };
                            } else if branch_val < min_branch.value { // Update min value
                                min_branch = BranchValue {
                                    piece_coordinates: piece_coordinates,
                                    move_coordinates: move_coordinates,
                                    value: branch_val,
                                };
                            }
                        }    
                    }
                }
            }
        }

        // If no child branches were searched every possible move at this depth was an invalid move
        // So set min and max value to branch min and max value
        if !init_min_max_branch {
            max = max_branch;
            min = min_branch;
        }

        if master_team { // Return max values for master team
            //println!("{}", max.value);
            // You got to update the god damn best_move function to get the checkmate
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

            assert_eq!(best_move(true, 0, 3, 0, game_state).move_coordinates, [7, 1]);
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

            assert_eq!(best_move(true, 0, 3, 0, game_state).move_coordinates, [3, 3]);
        }

        /*
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

            assert_eq!(best_move(true, 0, 4, 0, game_state), BranchValue::new());
        }
        */
    }
}