// Module for functions relating to conversion between chess, cartesian, and board coordinates
pub mod coordinate_conversion {

    // Converts chess coordinate notation (a2) to cartesian coordinates (x, y)
    pub fn chess_to_cartesian(board_number: Vec<char>) -> Result<Vec<i8>, ()> {
        // Turn chars into their respective ASCII values
        let x = board_number[0] as i8 - 97;
        let y = board_number[1] as i8 - 49;

        // Return an error if the converted coordinates are outside the range of a typical chess board
        if x > 7 || x < 0 || y > 7 || y < 0 {
            return Err(())
        }

        let engine_coordinates = vec![x, y];
        Ok(engine_coordinates)
    }

    
    pub fn cartesian_to_number(coordinate: Vec<i8>, grid_y_size: i8) -> i8 {
        // Converts cartesian coordinate to a square number
        // (1, 2) would convert to 7
        /*
            +---+---+---+
        2   | 6 | 7 | 8 | 
            +---+---+---+
        1   | 3 | 4 | 5 | 
            +---+---+---+
        0   | 0 | 1 | 2 |
            +---+---+---+
            
            0   1   2
        */

        // Types can be changed in function signature to suit a different application then chess
        // I just can't be bothered using generics

        //         y                             x
        coordinate[1] * grid_y_size + coordinate[0]
    }
}

#[cfg(test)]
mod helpers_tests {
    use crate::helpers;
    
    #[test]
    fn chess_to_cartesian() {
        let pos_vec = vec!['a', '2'];
        let result = helpers::coordinate_conversion::chess_to_cartesian(pos_vec).unwrap();

        let cartesian_vec = vec![0, 1];
        assert_eq!(result, cartesian_vec);
    }

    #[test]
    fn cartesian_to_number() {
        let cartesian_vec = vec![1, 2];
        let result = helpers::coordinate_conversion::cartesian_to_number(cartesian_vec, 3);

        assert_eq!(result, 7);
    }
}
