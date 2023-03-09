use std::io;
use std::collections::HashMap;

use chess::board::turn::GameState;
use chess::board::turn::PointsInfo;
use chess::piece::moves::BoardInfo;
use chess::board::BOARD_SIZE;
use chess::board::errors;

fn main() {

    // Initialize transposition table
    let mut transposition_table: HashMap<u64, chess::algorithm::minimax::TranspositionInfo> = HashMap::new();
    let bitstrings_board = chess::gen_bistrings_board();

    let player_white = false;

    let search_depth: usize = 4;

    // Get starting GameState
    let mut game_state = GameState {
        white_points_info: PointsInfo {
            captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
            captured_pieces_no: 0,
            points_total: 11,
            points_delta: 0,
        },

        black_points_info: PointsInfo {
            captured_pieces: [0i8; BOARD_SIZE[0] * {BOARD_SIZE[1] / 2}],
            captured_pieces_no: 0,
            points_total: 15,
            points_delta: 0,
        },

        points_delta: 0,

        board_info: BoardInfo {
            board: chess::fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            turns_board: [[0i8; BOARD_SIZE[0]]; BOARD_SIZE[0]],
            last_turn_coordinates: [0, 0],
            capture_coordinates: None,
            error_code: 0,
            pieces: chess::piece::info::Piece::instantiate_all(),
        },

        whites_turn: true,
    };

    let mut game_over = false;

    let mut player_turn = false;
    if player_white {
        player_turn = true;
    }

    // Continue to make moves untill the game is over
    while !game_over {
        println!("{}", transposition_table.len());

        // Initialize variables
        let mut game_state_new = Ok(game_state);

        let mut turn_error_struct = chess::board::turn::Error {
            game_over: false,
            white_win: None,
            error_code: 0,
            value: 0,
        };

        // Get the team name
        let mut team_name = "White";
        if !game_state.whites_turn {
            team_name = "Black";
        }

        // Get new game_state from player or ai, depending on who moved last turn
        let mut turn_error = true;
        while turn_error {

            // Print whos turn it is
            if player_white == game_state.whites_turn {
                println!("{}s turn", team_name);
            } else {
                println!("{}s turn (ai)", team_name);
            }
            

            if player_turn {
                let piece_coordinates = get_user_coordinates("Move piece at coordinates: ");
                let move_coordinates = get_user_coordinates("To new coordinates: ");
    
                game_state_new = chess::board::turn::new_turn(piece_coordinates, move_coordinates, chess::piece::info::IDS[4], game_state);
            } else {
                let best_move = chess::algorithm::minimax::best_move(true, 0, search_depth, 0, None, &bitstrings_board, &mut transposition_table, game_state);
                game_state_new = chess::board::turn::new_turn(best_move.piece_coordinates, best_move.move_coordinates, chess::piece::info::IDS[4], game_state);

                let piece_ccn = chess::cart_to_ccn(chess::flip_coordinates(best_move.piece_coordinates)).unwrap();
                let move_ccn = chess::cart_to_ccn(chess::flip_coordinates(best_move.move_coordinates)).unwrap();

                println!("{} to {}", piece_ccn, move_ccn);
                println!("{:?}", best_move);
                println!("");
            }
            
            // Set turn_error to false if the move was valid, or the game is over
            match game_state_new {
                Ok(game_state_unwrapped) => turn_error = false,
                Err(error) => {
                    if error.error_code == errors::CHECKMATE_ERROR || error.error_code == errors::STALEMATE_ERROR {
                        let error_message = errors::message(error.error_code);
                        println!("{}", error_message.unwrap());
                        turn_error = false;
                        turn_error_struct = error;
                    } else {
                        let error_message = errors::message(error.error_code);
                        println!("{}", error_message.unwrap());
                        println!("Please try again: ");
                        println!("");
                    }
                },
            };
        }

        // If the game is over give a game over message
        if turn_error_struct.game_over {
            game_over = true;
            match turn_error_struct.white_win {
                Some(white_win) => {
                    println!("{} team wins!", team_name);
                },
                None => println!("Tie!"),
            }
        } else { // Else update game state with the new one
            game_state = game_state_new.unwrap();
        }

        // Invert player_turn so the opposite team moves next move
        player_turn = !player_turn;
    }
}

// Get CCN coordinate from the user and convert it to standard cartesian coordinates
fn get_user_coordinates(message: &str) -> [i8; 2] {
    let coordinates = loop {
        println!("{}", message);

        // Get user input as a string
        let mut pos = String::new();
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read input");

        let pos_vec: Vec<char> = pos.chars().collect(); // Convert string to a vector of chars
        let result = chess::ccn_to_cart(pos_vec);

        // If the chess_to_engine function fails, provide an error and allow the user to try again
        let coordinates = match result {
            Ok(c) => c,
            Err(_) => {
                println!("Please use correct coordinate formatting (E.g. a1). This is case sensetive.");
                println!();
                continue
            },
        };

        println!();
        break coordinates
    };

    coordinates
}