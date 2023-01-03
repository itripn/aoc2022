use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn cycle_aligned( cycle : u32 ) -> bool {

    (cycle == 20) || ((cycle % 40) == 20)
}

fn run_instruction_set( instructions : &[String] ) {

    let mut cycle = 1;
    let mut accum: i32= 1;
    let mut signal_strength: i32 = 0;


    for instr in instructions {

        if instr.starts_with( "noop" ) {
            if cycle_aligned( cycle) {
                signal_strength += accum * cycle as i32;   
            }
            cycle += 1;
        }
        else if instr.starts_with( "addx" ) {

            let arg : i32 = instr[5..].parse().unwrap();

            if cycle_aligned( cycle ) {
                println!("cycle1: {}", cycle);
                signal_strength += accum * cycle as i32;
            }
            else if cycle_aligned(cycle + 1) {
                println!("cycle2: {}", cycle+1);
                signal_strength += accum * (cycle as i32 +1);
            }
            accum += arg;
            cycle += 2;

            // if cycle_aligned(cycle) {
            //     println!("cycle3: {}", cycle);
            //     signal_strength += accum * cycle as i32;
            // }

        }
    }

    println!("Total signal Strength: {}", signal_strength );

}

fn main() {

    let mut instructions : Vec<String> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.is_empty() {

                instructions.push( line );
            }
        }
    }

    run_instruction_set( &instructions );

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
