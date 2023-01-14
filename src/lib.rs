pub mod fen;
pub mod piece;
pub mod board;

// Converts chess coordinate notation (a2) to cartesian coordinates (x, y)
pub fn ccn_to_cart(board_number: Vec<char>) -> Result<[i8; 2], ()> {

    // Turn chars into their respective ASCII values
    let x = board_number[0] as i8 - 97;
    let y = board_number[1] as i8 - 49;

    // Get board x and y max coordinate
    let bx = board::BOARD_SIZE[0] - 1;
    let by = board::BOARD_SIZE[1] - 1;

    // Return an error if the converted coordinates are outside the range of a typical chess board
    if x > bx || x < 0 || y > by || y < 0 {
        return Err(())
    }

    let engine_coordinates: [i8; 2] = [x, y];
    Ok(engine_coordinates)
}

// Converts cartesian to linear coordinates
pub fn cart_to_lin(cart: [i8; 2]) -> i8 {
    cart[1] * board::BOARD_SIZE[1] + cart[0]
}

// Converts linear to cartesian coordinates
pub fn lin_to_cart(lin: i8) -> [i8; 2] {
    let x = lin % board::BOARD_SIZE[0];
    let y = lin / board::BOARD_SIZE[1];
    [x, y]
}
// Return true if a char is uppercase
pub fn uppercase_char(c: char) -> bool {
    if c as i8 > 90 {
        return false;
    }
    true
}

// Convert lowercase char to uppercase char
pub fn lower_to_upper_char(c: char) -> char {
    let c_upper = c as u8 - 32;
    c_upper as char
}

#[cfg(test)]
mod tests {
    use super::*;

    mod coordinate_conversion_tests {
        use super::*;

        #[test]
        fn ccn_to_cart_test() {
            let pos_vec = vec!['a', '2'];
            let result = ccn_to_cart(pos_vec).unwrap();

            assert_eq!(result, [0, 1]);
        }

        #[test]
        fn cart_to_lin_test() {
            let result = cart_to_lin([3, 4]);

            assert_eq!(result, 35);
        }

        #[test]
        fn lin_to_cart_test() {
            let result = lin_to_cart(58);

            assert_eq!(result, [2, 7]);
        }

    }

    #[test]
    fn lower_to_upper_char_test() {
        assert_eq!(lower_to_upper_char('a'), 'A');
    }
    

}
