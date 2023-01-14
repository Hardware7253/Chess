use std::io;

// Definitions for terminology used accross the project
/*  
    // LIN
    // Used in names like move_lin
    Linear coordinate, usually abreviated to lin refers to the coordinate system this engine uses for the board.
    It works by giving each square a uniqie number (or its linear coordinate). These numbers start from 0 at the bottom right of the board.
    
    //A representation of LIN coordinates on a chess board, with CCN coordinates along the sides
    8   56, 57, 58, 59, 60, 61, 62, 63
    7   48. 49, 50, 51, 52, 53, 54, 55,
    6   40, 41, 42, 43, 44, 45, 46, 47,
    5   32, 33, 34, 35, 36, 37, 38, 39
    4   24, 25, 26, 27, 28, 29, 30, 31
    3   16, 17, 18, 19, 20, 21, 22, 23,
    2   08, 09, 10, 11, 12, 13, 14, 15,
    1   00, 01, 02, 03, 04, 05, 06, 07,

        A,  B,  C,  D,  E,  F,  G,  H

    This system is used so the board can easily be stored in a standard 1 dimensional array

    // CART
    // Used in names like: move_cart
    Cartesian (x, y) coordinates, usually abreviated to cart.

    // CCN
    // Used in names like ccn_to_cart
    Chess coordinate notation (E.g. a2), usually abreviated to ccn

    // BOARD
    // Used in names like board
    Board is a chess board array, the array is indexed by a LIN coordinate

    // DLIN, DCART
    // Used in names like: dlin, dcart
    Shorthand for delta lin, or delta cart. It is used when a transformation is being applied to a coordinate

*/



fn main() {
    // When using custom fen strings to initiate a game consider how move will affect the game.
    // E.g. a pawn can en passant when it shouldn't be able to
    // Or a castle can be initiated when it shouldn't be able to
    // Check custom fen strings with defualt fen strings to tell if piece have moved
    // Giberish

    // Get one pieces array to use for the entire program
    let pieces = chess::piece::info::Piece::instantiate_all();

    //let board = board::fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let board = chess::fen::decode("8/8/6P1/8/8/8/8/8");
    println!("{:?}", board);
    
    let coordinates = get_user_coordinates("Enter a coordinate:");
    println!("{:?}", coordinates);
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