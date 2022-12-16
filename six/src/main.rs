use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn count_chars(seq: &str, tc: char) -> u32 {
    // println!("checking: {}", seq );
    let mut n = 0;
    for c in seq.chars() {
        if c == tc {
            n += 1;
        }
    }

    n
}

fn is_start_of_packet(seq: &str) -> bool {
    for c in seq.chars() {
        if count_chars(seq, c) > 1 {
            return false;
        }
    }

    true
}

fn find_start_of_packet(line: &str, pkt_size: usize) -> usize {
    let mut i: usize = 0;

    while i + 4 < line.len() {
        if is_start_of_packet(&line[i..=i + pkt_size - 1]) {
            return i + pkt_size;
        }

        i += 1;
    }

    0
}

fn main() {
    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.is_empty() {
                let sop: usize = find_start_of_packet(&line, 4);
                println!("Start of packet: {}", sop);

                let som: usize = find_start_of_packet(&line, 14);
                println!("Start of message: {}", som);
            }
        }
    }
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
