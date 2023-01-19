pub mod fen;
pub mod piece;
pub mod board;

// Converts chess coordinate notation (a2) to cartesian coordinates (x, y)
pub fn ccn_to_cart(ccn: Vec<char>) -> Result<[i8; 2], ()> {

    // Turn chars into their respective ASCII values
    let x = ccn[0] as i8 - 97;
    let y = ccn[1] as i8 - 49;

    // Get board max coordinates
    let bx = board::BOARD_SIZE[0] - 1;
    let by = board::BOARD_SIZE[1] - 1;

    let bx: i8 = bx.try_into().unwrap();
    let by: i8 = by.try_into().unwrap();

    // Return an error if the converted coordinates are outside the range of a typical chess board
    if x > bx || x < 0 || y > by || y < 0 {
        return Err(())
    }

    let cart: [i8; 2] = [x, y];
    Ok(cart)
}

// Return true if a char is uppercase
pub fn uppercase_char(c: char) -> bool {
    let c_num = c as i8;
    if c_num > 90 || c_num < 65 {
        return false;
    }
    true
}

// Convert lowercase char to uppercase char
pub fn lower_to_upper_char(c: char) -> char {
    let c_upper = c as u8 - 32;
    if c_upper  > 90 || c_upper < 65 {
        panic!("Lower case char not provided");
    }
    c_upper as char
}

// Convert a char of a number to an integer
// E.g. '1' -> 1
pub fn char_to_num(c: char) -> Result<i8, ()> {
    let num = c as i8 - 48;
    if num < 0 || num > 9 {
        return Err(())
    }
    Ok(num)
}

// Unwrap an Option<T>, if there is not a Some() value return the def parameter
pub fn unwrap_def<T>(option: Option<T>, def: T) -> T {
    match option {
        Some(t) => t,
        None => def,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ccn_to_cart_test() {
        let pos_vec = vec!['a', '2'];
        let result = ccn_to_cart(pos_vec).unwrap();

        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn uppercase_char_test() {
        assert_eq!(uppercase_char('A'), true);
    }

    #[test]
    fn lower_to_upper_char_test() {
        assert_eq!(lower_to_upper_char('a'), 'A');
    }

    #[test]
    fn char_to_num_test() {
        assert_eq!(char_to_num('0'), Ok(0));
    }

    #[test] 
    fn unwrap_def_test() {
        let num: i8 = 2;
        let num_option: Option<i8> = Some(num);

        assert_eq!(unwrap_def(num_option, 0), num);
    }
}
