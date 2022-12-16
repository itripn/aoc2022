use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3
//
fn _get_initial_state_test() -> Vec<VecDeque<char>> {
    let one = VecDeque::from(['N', 'Z']);
    let two = VecDeque::from(['D', 'C', 'M']);
    let three = VecDeque::from(['P']);

    Vec::from([one, two, three])
}

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

fn get_initial_state() -> Vec<VecDeque<char>> {
    let one = VecDeque::from(['R', 'Q', 'G', 'P', 'C', 'F']);
    let two = VecDeque::from(['P', 'C', 'T', 'W']);
    let three = VecDeque::from(['C', 'M', 'P', 'H', 'B']);
    let four = VecDeque::from(['R', 'P', 'M', 'S', 'Q', 'T', 'L']);
    let five = VecDeque::from(['N', 'G', 'V', 'Z', 'J', 'H', 'P']);
    let six = VecDeque::from(['J', 'P', 'D']);
    let seven = VecDeque::from(['R', 'T', 'J', 'F', 'Z', 'P', 'G', 'L']);
    let eight = VecDeque::from(['J', 'T', 'P', 'F', 'C', 'H', 'L', 'N']);
    let nine = VecDeque::from(['W', 'C', 'T', 'H', 'Q', 'Z', 'V', 'G']);

    Vec::from([one, two, three, four, five, six, seven, eight, nine])
}

fn execute_step(line: &str, state: &mut [VecDeque<char>], part2: bool) {
    // move <count> from <stack #> to <stack #>
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let caps = RE.captures(line).unwrap();
    let count = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
    let fstack = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap()) - 1;
    let tstack = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap()) - 1;

    if part2 {
        let mut temp: VecDeque<char> = state[fstack].drain(0..count).collect::<VecDeque<char>>();
        temp.make_contiguous().reverse();
        temp.iter()
        .for_each(|c| state[tstack].push_front(*c));

    } else {
        for _i in 0..count {
            let c = state[fstack].pop_front().unwrap();
            state[tstack].push_front(c);
        }
    }
}

fn _print_stacks(containers: &[VecDeque<char>]) {
    println!("\n===");
    for c in containers {
        println!("{:?}", *c);
    }
    println!("===\n");
}

fn main() {
    let mut containers = get_initial_state();
    let mut p2containers = get_initial_state();

    // print_stacks(&containers);
    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.is_empty() {
                execute_step(&line, &mut containers, false);
                execute_step(&line, &mut p2containers, true);
            }
        }
    }

    print!("\n Part 1: ");
    containers.into_iter().for_each(|c| print!("{}", c[0]));

    print!("\n Part 2: ");
    p2containers.into_iter().for_each(|c| print!("{}", c[0]));

    println!("\n");
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
