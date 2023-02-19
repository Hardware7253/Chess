pub mod minimax {
    use crate::board::turn::new_turn;
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

    struct BranchBest {
        move_coordinates: [i8; 2],
        value: i8,
    }

    // Double check pawn cannot jump pieces when doing double move

    // Get best move purely based on material (future moves not accounted for)
    fn best_move_material(piece_coordinates: [i8; 2], master_team: bool, init_val: i8, search_depth: usize, current_depth: usize, game_state: GameState) -> BranchBest {
        use crate::board::turn::new_turn;
        use crate::coordinates_from_usize;
        let starting_team = game_state.whites_turn;

        if search_depth == current_depth {
            return BranchBest {
                move_coordinates: piece_coordinates,
                value: init_val,
            }
        } else {

            let mut branch_min = BranchBest {
                move_coordinates: [0, 0],
                value: 0,
            };

            let mut branch_max = BranchBest {
                move_coordinates: [0, 0],
                value: 0,
            };

            for x in 0..BOARD_SIZE[0] {
                for y in 0..BOARD_SIZE[1] {
                    let move_coordinates = coordinates_from_usize([x, y]);
                    let mut check_child = true;

                    // Get the material value from moving to new coordinates
                    let game_state_new = new_turn(piece_coordinates, move_coordinates, game_state);
                    let mut move_val = match game_state_new {
                        Ok(game_state) => game_state.points_delta,
                        Err(error) => {check_child = false; error.value}, // If there is an error there is no reason to check child branches
                    };
                    
                    // If the current branch is not the master team then it's move values are negative
                    if !master_team {
                        move_val *= -1;
                    }

                    if move_val > 0 {
                        //println!("{:?}", game_state_new.unwrap().board_info.board);
                        //println!("{}", move_val);
                    }

                    

                    /*
                    
                    */
    
                    let branch_val = init_val + move_val;

                    if init_val == 3 {
                        //println!("{}", move_val);
                        //println!("{}", branch_val);
                    }
                    
                    if check_child {
                        for check_x in 0..BOARD_SIZE[0] {
                            for check_y in 0..BOARD_SIZE[1] {
                                let piece_coordinates = coordinates_from_usize([check_x, check_y]);
    
                                let child_min_max = best_move_material(piece_coordinates, !master_team, branch_val, search_depth, current_depth + 1, game_state_new.unwrap());

                                // Initialize min and max values with first value
                                if check_x == 0 && check_y == 0 {
                                    branch_max.value = child_min_max.value;
                                    branch_min.value = child_min_max.value;

                                    branch_max.move_coordinates = move_coordinates;
                                }
                                    //println!("{}", child_min_max.value);
                                    //println!("{}", )
    
                                // Update min and max values
                                if child_min_max.value > branch_max.value {
                                    branch_max.value = child_min_max.value;
                                    branch_max.move_coordinates = move_coordinates;
                                } if child_min_max.value < branch_min.value {
                                    branch_min.value = child_min_max.value;
                                    branch_min.move_coordinates = move_coordinates;
                                }
                            }
                        }
                    }        
                }
            }

            if current_depth == 1 {
                //println!("{}", branch_max.value);
                //println!("{}", branch_min.value);
            }
            

            let mut return_value = 0;
            if master_team {
                
                return branch_max;
            } else {
                //println!("{}", branch_min.value);
                //println!("{}", current_depth);
                return branch_min;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::fen;
        use crate::board::turn::PointsInfo;
        use crate::piece::moves::BoardInfo;
        use super::*;

        #[test]
        fn best_move_material_test() {
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
                    board: fen::decode("8/8/8/8/2b5/3r4/3R3n/8"),
                    turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            assert_eq!(best_move_material([3, 1], true, 0, 2, 0, game_state).move_coordinates, [7, 1]);
        }
    }
}