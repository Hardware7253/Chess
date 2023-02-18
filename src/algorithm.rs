pub mod minimax {
    use crate::board::turn::GameState;
    use crate::board::BOARD_SIZE;

    /*
    // Given a game state returns the best possible move (that can be found)
    pub fn best_move(piece_coordinates: [i8; 2], search_depth_start: usize, search_depth: usize, game_state: GameState) {
        //let board_squares = BOARD_SIZE[0] * BOARD_SIZE[1]; // Get number of squares on the board
        //let possible_branches = board_squares.pow((search_depth + 1).try_into().unwrap()); // Get number of possible branches (most will be invalid moves)

        //let mut board_levels = [[[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]; ]; // Stores current board for each branch level
        //let mut board_levels = vec![[[0i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]];
        //let mut board_levels: Vec<[[i8; BOARD_SIZE[0]]; BOARD_SIZE[1]]> = Vec::new();

        //let mut branch_coordinates: Vec<[i8; 2]> = Vec::new();
        //let mut branch_states: Vec<GameState> = Vec::new();

        for x in 0..BOARD_SIZE[0] {
            for y in 0..BOARD_SIZE[1] {
                let move_coordinates = crate::coordinates_from_usize([x, y]);
            }
        }
        
    }
    */

    // Get best move purely based on material (future moves not accounted for)
    fn best_move_material(piece_coordinates: [i8; 2], master_team: bool, game_state: GameState) -> i8 {
        use crate::board::turn::new_turn;
        let starting_team = game_state.whites_turn;

        let mut best_val = 0;
        for x in 0..BOARD_SIZE[0] {
            for y in 0..BOARD_SIZE[1] {
                let move_coordinates = crate::coordinates_from_usize([x, y]);

                let game_state_new = new_turn(piece_coordinates, move_coordinates, game_state);
                let move_val = match game_state_new {
                    Ok(game_state) => game_state.points_delta,
                    Err(error) => error.value,
                };

                if move_val > best_val {
                    best_val = move_val;
                }
            }
        }

        if !master_team {
            best_val *= -1;
        }

        best_val
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
                    board: fen::decode("8/8/8/8/2n1p3/3P4/8/8"),
                    turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
                    last_turn_coordinates: [0, 0],
                    capture_coordinates: None,
                    error_code: 0,
                    pieces: crate::piece::info::Piece::instantiate_all(),
                },

                whites_turn: true,
            };

            assert_eq!(best_move_material([3, 2], true, game_state), 3);
        }
    }
}