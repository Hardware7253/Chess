use std::io;

fn trash() {
        /*
    struct Piece {
        name: String,
        identifier: u8,
        value: u8,

        white: Option<bool>,

        sliding: bool,
        coordinate: Option<i8>,
        movement_directions: [i8; 8],
        capture_directions: [i8; 8],

        move_count: u32,
    }

    impl Piece {
        fn pawn() -> Piece {
            Piece {
                name: String::from("Pawn"),
                identifier: 1,
                value: 1,
        
                white: None,
        
                sliding: false,
                coordinate: None,
                movement_directions: [8, 0, 0, 0, 0, 0, 0, 0],
                capture_directions: [-7, 9, 0, 0, 0, 0, 0 ,0],
        
                move_count: 0,
            }
        }

        fn rook() -> Piece {
            Piece {
                name: String::from("Rook"),
                identifier: 2,
                value: 5,
        
                white: None,
        
                sliding: true,
                coordinate: None,
                movement_directions: [1, -1, 8, -8, 0, 0, 0, 0],
                capture_directions: [1, -1, 8, -8, 0, 0, 0, 0],
        
                move_count: 0,
            }
        }

        fn knight() -> Piece {
            Piece {
                name: String::from("Knight"),
                identifier: 3,
                value: 3,
        
                white: None,
        
                sliding: false,
                coordinate: None,
                movement_directions: [17, -17, 15, -15, 6, -6, 10, -10],
                capture_directions: [17, -17, 15, -15, 6, -6, 10, -10],
        
                move_count: 0,
            }
        }

        fn bishop() -> Piece {
            Piece {
                name: String::from("Bishop"),
                identifier: 4,
                value: 3,
        
                white: None,
        
                sliding: true,
                coordinate: None,
                movement_directions: [9, -9, 7, -7, 0, 0, 0, 0],
                capture_directions: [9, -9, 7, -7, 0, 0, 0, 0],
        
                move_count: 0,
            }
        }

        fn queen() -> Piece {
            Piece {
                name: String::from("Queen"),
                identifier: 5,
                value: 9,
        
                white: None,
        
                sliding: true,
                coordinate: None,
                movement_directions: [1, -1, 8, -8, 9, -9, 7, -7],
                capture_directions: [1, -1, 8, -8, 9, -9, 7, -7],
        
                move_count: 0,
            }
        }

        fn king() -> Piece {
            Piece {
                name: String::from("King"),
                identifier: 6,
                value: 255,
        
                white: None,
        
                sliding: false,
                coordinate: None,
                movement_directions: [1, -1, 8, -8, 9, -9, 7, -7],
                capture_directions: [1, -1, 8, -8, 9, -9, 7, -7],
        
                move_count: 0,
            }
        }
    }
    */

    /*
    struct Piece {
        value: i8,

        sliding: bool,
        movement_directions: [i8; 8],
        capture_directions: [i8; 8],

        move_count: i32,
    }

    impl Piece {
        fn pawn() -> Piece {
            Piece {
                value: 1,
        
                sliding: false,
                movement_directions: [8, 0, 0, 0, 0, 0, 0, 0],
                capture_directions: [-7, 9, 0, 0, 0, 0, 0 ,0],
        
                move_count: 0,
            }
        }

        fn rook() -> Piece {
            Piece {
                value: 5,
        
                sliding: true,
                movement_directions: [1, -1, 8, -8, 0, 0, 0, 0],
                capture_directions: [1, -1, 8, -8, 0, 0, 0, 0],
        
                move_count: 0,
            }
        }

        fn knight() -> Piece {
            Piece {
                value: 3,

                sliding: false,
                movement_directions: [17, -17, 15, -15, 6, -6, 10, -10],
                capture_directions: [17, -17, 15, -15, 6, -6, 10, -10],
        
                move_count: 0,
            }
        }

        fn bishop() -> Piece {
            Piece {
                value: 3,
        
                sliding: true,
                movement_directions: [9, -9, 7, -7, 0, 0, 0, 0],
                capture_directions: [9, -9, 7, -7, 0, 0, 0, 0],
        
                move_count: 0,
            }
        }

        fn queen() -> Piece {
            Piece {
                value: 9,

                sliding: true,
                movement_directions: [1, -1, 8, -8, 9, -9, 7, -7],
                capture_directions: [1, -1, 8, -8, 9, -9, 7, -7],
        
                move_count: 0,
            }
        }

        fn king() -> Piece {
            Piece {
                value: 127,

                sliding: false,
                movement_directions: [1, -1, 8, -8, 9, -9, 7, -7],
                capture_directions: [1, -1, 8, -8, 9, -9, 7, -7],
        
                move_count: 0,
            }
        }
    }
    */
}



fn main() {
    // When using custom fen strings to initiate a game consider how move will affect the game.
    // E.g. a pawn can en passant when it shouldn't be able to
    // Or a castle can be initiated when it shouldn't be able to
    // Check custom fen strings with defualt fen strings to tell if piece have moved
    // Giberish

    //let board = board::fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let board = chess::fen::decode("8/8/6P1/8/8/8/8/8");
    println!("{:?}", board);
    
    let num = get_user_coordinates("Enter a coordinate:");
    println!("{}", num);
}

// Get coordinate input from the user and convert it to chess coordinates
fn get_user_coordinates(message: &str) -> i8 {
    let cartesian_coordinates = loop {
        println!("{}", message);

        // Get user input as a string
        let mut pos = String::new();
        io::stdin()
            .read_line(&mut pos)
            .expect("Failed to read input");

        let pos_vec: Vec<char> = pos.chars().collect(); // Convert string to a vector of chars
        let result = chess::helpers::coordinate_conversion::chess_to_cartesian(pos_vec); // Use chess_to_cartesian to convert chess coordinate notation to cartesian coordinates

        // If the chess_to_engine function fails provide an error and allow the user to try again
        let coordinates = match result {
            Ok(c) => c,
            Err(e) => {
                println!("Please use correct coordinate formatting (E.g. a1). This is case sensetive.");
                println!();
                continue
            },
        };

        println!();
        break coordinates
    };
    chess::helpers::coordinate_conversion::cartesian_to_number(cartesian_coordinates, 8) // Convert cartesian coordinates to board coordinates
}

    /*

    // Chess coordinates
        +---+---+---+---+---+---+---+---+
    8   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    7   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    6   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    5   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    4   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    3   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    2   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    1   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
         
          a   b   c   d   e   f   g   h


    // Cartesian coordinates
        +---+---+---+---+---+---+---+---+
    7   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    6   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    5   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    4   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    3   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    2   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    1   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
    0   |   |   |   |   |   |   |   |   |
        +---+---+---+---+---+---+---+---+
         
          0   1   2   3   4   5   6   7

    */