use std::io;

fn main() {
    // When using custom fen strings to initiate a game consider how move will affect the game.
    // E.g. a pawn can en passant when it shouldn't be able to
    // Or a castle can be initiated when it shouldn't be able to
    // Check custom fen strings with defualt fen strings to tell if piece have moved

    // Get one pieces array to use for the entire program
    let pieces = chess::piece::info::Piece::instantiate_all();

    //let board = chess::fen::decode("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let board = chess::fen::decode("8/7P/6P1/P1P2P2/P1P1PR2/PPPP1R2/PP1PP3/1PPPPPPP");
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