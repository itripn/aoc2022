
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Input
//
//                         [R] [J] [W]
//             [R] [N]     [T] [T] [C]
// [R]         [P] [G]     [J] [P] [T]
// [Q]     [C] [M] [V]     [F] [F] [H]
// [G] [P] [M] [S] [Z]     [Z] [C] [Q]
// [P] [C] [P] [Q] [J] [J] [P] [H] [Z]
// [C] [T] [H] [T] [H] [P] [G] [L] [V]
// [F] [W] [B] [L] [P] [D] [L] [N] [G]
//  1   2   3   4   5   6   7   8   9 

fn get_initial_state() -> Vec<Vec<char>> {

    let one   = Vec::from( ['R','Q','G','P','C','F'] );
    let two   = Vec::from( ['P', 'C', 'T', 'W'] );
    let three = Vec::from( ['C', 'M', 'P', 'H', 'B']);
    let four  = Vec::from( ['R', 'P', 'M', 'S', 'Q', 'T', 'L'] );
    let five  = Vec::from( ['N', 'G', 'V', 'Z', 'J', 'H', 'P'] );
    let six   = Vec::from( ['J', 'P', 'D'] );
    let seven = Vec::from( ['R', 'T', 'J', 'F', 'Z', 'P', 'G', 'L'] );
    let eight = Vec::from( ['J', 'T', 'P', 'F', 'C', 'H', 'L', 'N'] );
    let nine  = Vec::from( ['W', 'C', 'T', 'H', 'Q', 'Z', 'V', 'G'] );
   
    Vec::from([one, two, three, four, five, six, seven, eight, nine])
}


fn main() {

    let mut containers = get_initial_state();

    
    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    // let row : Vec<char> = l.chars().collect();
                    // data.push( row );
                    // println!( "{}", l );
                }
            }
        }
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
