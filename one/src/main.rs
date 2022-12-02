
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_one( calories : &Vec<u32> ) {

    let largest: u32 = calories.iter().fold(std::u32::MIN, |a,b| a.max(*b));
    println!("\nPart 1: elf with most calories: {}", largest );
}

fn part_two( calories : Vec<u32> ) {

    let mut mvec = calories.to_vec();
    mvec.sort();
    
    let top3 = mvec.iter().take(3);
    let total = top3.fold( 0, |acc, n| acc + n );
    println!("Sum of top 3 calories: {}", total);
}

fn main() {

    let mut elf_calories : Vec<u32> = Vec::new();

    // let mut data : Vec<Vec<char>> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        let mut running_total = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() == 0 {
                    elf_calories.push(running_total);
                    running_total = 0;
                }
                else {
                    running_total = running_total + ( l.parse::<u32>().unwrap() );
                }
            }
        }
    }

    part_one( &elf_calories );
    part_two ( elf_calories );

}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
