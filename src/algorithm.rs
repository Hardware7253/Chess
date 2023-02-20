pub mod minimax {
    use crate::board::turn::new_turn;
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

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

        fn update_coordinates(piece_coordinates: [i8; 2], move_coordinates: [i8; 2], mut branch_value: BranchValue) -> BranchValue {
            branch_value.piece_coordinates = piece_coordinates;
            branch_value.move_coordinates = move_coordinates;

            branch_value
        }
    }

    fn best_move(master_team: bool, init_val: i8, search_depth: usize, current_depth: usize, game_state: GameState) -> BranchValue {
        use crate::coordinates_from_usize;
        use crate::board::errors;

        

        if current_depth == search_depth {
            return BranchValue {
                piece_coordinates: [0, 0],
                move_coordinates: [0, 0],
                value: init_val,
            };
        }

        let mut max_val = 0;
        let mut max_coordinates = BranchValue::new();

        let mut min_val = 0;
        let mut min_coordinates = BranchValue::new();

        let mut max_branch_val = 0;
        let mut max_branch_coordinates = BranchValue::new();

        let mut min_branch_val = 0;
        let mut min_branch_coordinates = BranchValue::new();

        let mut init_min_max_val = true;
        let mut init_min_max_branch_val = true;


        let mut searched_down = false; // False when children branches haven't been searched
        for x_piece in 0..BOARD_SIZE[0] {
            for y_piece in 0..BOARD_SIZE[1] {
                let piece_coordinates = coordinates_from_usize([x_piece, y_piece]);

                for x_move in 0..BOARD_SIZE[0] {
                    for y_move in 0..BOARD_SIZE[1] {
                        let move_coordinates = coordinates_from_usize([x_move, y_move]);

                        let mut move_error = false;
                        let mut valid_move = true;

                        // Get the material value of moving from piece_coordinates to move_coordinates
                        let game_state_new = new_turn(piece_coordinates, move_coordinates, game_state);
                        let mut move_val = match game_state_new {
                            Ok(game_state) => game_state.points_delta,
                            Err(error) => {
                                move_error = true;

                                // If the error was not a checkmate, or stalemate then the error was related to an invalid move
                                if error.error_code != errors::CHECKMATE_ERROR && error.error_code != errors::STALEMATE_ERROR {
                                    valid_move = false;
                                }

                                error.value}, // If there is an error there is no reason to check child branches
                        };

                        // If there is a stalemate, preferentially avoid it for another move of the same value

                        // If the current branch is not the master team then it's move values are negative (because they negatively impact the master team)
                        if !master_team {
                            move_val *= -1
                        }

                        let branch_val = init_val + move_val;

                        if !move_error {
                            searched_down = true;

                            let child_min_max = best_move(!master_team, branch_val, search_depth, current_depth + 1, game_state_new.unwrap());
                            
                            // Update min and max for child value
                            if init_min_max_val { // Initialize max and min value
                                max_val = child_min_max.value;
                                max_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, max_coordinates);

                                min_val = child_min_max.value;
                                min_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, min_coordinates);

                                init_min_max_val = false;
                            } else if child_min_max.value > max_val { // Update max value
                                max_val = child_min_max.value;
                                max_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, max_coordinates);
                            } else if child_min_max.value < min_val { // Update min value
                                min_val = child_min_max.value;
                                min_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, min_coordinates);
                            }

                        } else if valid_move { // Only update branch min max for valid moves (so no invalid moves are returned)

                            // Update min and max for branch value
                            if init_min_max_branch_val { // Initialize max and min value
                                max_branch_val = branch_val;
                                max_branch_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, max_coordinates);

                                min_branch_val = branch_val;
                                min_branch_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, min_coordinates);

                                init_min_max_branch_val = false;
                            } else if branch_val > max_branch_val { // Update max value
                                max_branch_val = branch_val;
                                max_branch_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, max_coordinates);
                            } else if branch_val < min_branch_val { // Update min value
                                min_branch_val = branch_val;
                                min_branch_coordinates = BranchValue::update_coordinates(piece_coordinates, move_coordinates, min_coordinates);
                            }
                        }    
                    }
                }
            }
        }

        // If no child branches were searched every possible move at this depth was an invalid move
        // So sit min and max value to branch min and max value
        if !searched_down {
            max_val = max_branch_val;
            min_val = min_branch_val;

            max_coordinates = max_branch_coordinates;
            min_coordinates = min_branch_coordinates;
        }

        if master_team { // Return max values for master team
            return BranchValue {
                piece_coordinates: max_coordinates.piece_coordinates,
                move_coordinates: max_coordinates.move_coordinates,
                value: max_val,
            };
        }

        BranchValue { // Return min values for not master team
            piece_coordinates: min_coordinates.piece_coordinates,
            move_coordinates: min_coordinates.move_coordinates,
            value: min_val,
        }
        
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

            assert_eq!(best_move(true, 0, 2, 0, game_state).move_coordinates, [7, 1]);
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

            assert_eq!(best_move(true, 0, 2, 0, game_state).move_coordinates, [3, 3]);
        }
    }
}