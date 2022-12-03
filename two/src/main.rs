
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_one( moves : &Vec<String> ) {

    let total_points = moves.iter().fold(0, |acc, m| {

        let mut points = 0;
        let mut parts = m.split(" ");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        match (a,b) {
            ("A","X") => { points = points + 1 + 3; }, // Draw
            ("A","Y") => { points = points + 2 + 6; }, // Won
            ("A","Z") => { points = points + 3 + 0; }, // Lost
            ("B","X") => { points = points + 1 + 0; }, // Lost
            ("B","Y") => { points = points + 2 + 3; }, // Draw
            ("B","Z") => { points = points + 3 + 6; }, // Win
            ("C","X") => { points = points + 1 + 6; }, // Win
            ("C","Y") => { points = points + 2 + 0; }, // Lost
            ("C","Z") => { points = points + 3 + 3; }, // Draw
            _ => { points = 0; }
        }
        acc + points 
    });

    println!("Total points: {}", total_points );

}

fn part_two( moves : &Vec<String> ) {

    let total_points = moves.iter().fold(0, |acc, m| {

        let mut points = 0;
        let mut parts = m.split(" ");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        match (a,b) {
            ("A","X") => { points = points + 3 + 0; }, // Need to lose (scissors)
            ("A","Y") => { points = points + 1 + 3; }, // Need to draw (rock)
            ("A","Z") => { points = points + 2 + 6; }, // Need to win (paper)
            ("B","X") => { points = points + 1 + 0; }, // Need to loose (rock)
            ("B","Y") => { points = points + 2 + 3; }, // Need to draw (paper)
            ("B","Z") => { points = points + 3 + 6; }, // Need to win (scissors)
            ("C","X") => { points = points + 2 + 0; }, // Need to lose (paper)
            ("C","Y") => { points = points + 3 + 3; }, // Need to draw (scissors)
            ("C","Z") => { points = points + 1 + 6; }, // Need to win (rock)
            _ => { points = 0; }
        }
        acc + points 
    });

    println!("Total points: {}", total_points );

}
fn main() {

    let mut moves : Vec<String> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    moves.push( l );
                }
            }
        }
    }

    part_one(&moves);
    part_two(&moves);
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
