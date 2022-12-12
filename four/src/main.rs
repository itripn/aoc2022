
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::path::Path;
use regex::Regex;


    // let a = 3..12;
    // let b = 4..12;
    //
    // println!("a len: {}", a.len());
    // println!("b len: {}", b.len());
    //
    // let aset : HashSet<u32> = a.collect();
    // let bset : HashSet<u32> = b.collect();
    //
    // let intersection = aset.intersection(&bset);
    //
    // println!("{:?}", intersection);
    // let isec : Vec<u32> = intersection.map(|n| *n ).collect();
    // let bvec = Vec::from_iter(bset);
    //
    // assert!( bvec == isec, "not equal!");


fn parse_ranges( assignments : &String ) -> Vec<u32> {

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let foo = re.captures( assignments ).unwrap();


    let sval1 = foo.get(1).map_or("", |m| m.as_str());
    let sval2 = foo.get(2).map_or("", |m| m.as_str());
    let sval3 = foo.get(3).map_or("", |m| m.as_str());
    let sval4 = foo.get(4).map_or("", |m| m.as_str());

    return [
        sval1.parse::<u32>().unwrap(),
        sval2.parse::<u32>().unwrap(),
        sval3.parse::<u32>().unwrap(),
        sval4.parse::<u32>().unwrap(),
    ].to_vec();

}

fn ranges_fully_contain( range_bounds : &Vec<u32> ) -> bool {

    let r1 = RangeInclusive::new( range_bounds[0], range_bounds[1]);
    let r2 = RangeInclusive::new( range_bounds[2], range_bounds[3]);
    let aset : HashSet<u32> = r1.collect();
    let bset : HashSet<u32> = r2.collect();   
    let intersection = aset.intersection(&bset);   
    let isec : Vec<u32> = intersection.map(|n| *n ).collect();

    let bvec = if aset.len() > bset.len() {
        Vec::from_iter(bset.clone())
    }
    else {
        Vec::from_iter(aset.clone())
    };
    
    bvec == isec
}

fn ranges_overlap_at_all( range_bounds : &Vec<u32> ) -> bool {

    let r1 = RangeInclusive::new( range_bounds[0], range_bounds[1] );
    let r2 = RangeInclusive::new( range_bounds[2], range_bounds[3] );

    // Lazy but easy...
    //
    r1.contains(r2.start()) || r1.contains(r2.end()) || r2.contains(r1.start()) || r2.contains(r1.end())
}

fn main() {

    // Each line is: 71-97,71-72

    let mut fully_overlapping_assignment_count = 0u32;
    let mut any_overlapping_assignment_count = 0u32;


    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    // let row : Vec<char> = l.chars().collect();
                    // data.push( row );
                    let range_bounds = parse_ranges(&l);
                    if ranges_fully_contain( &range_bounds ) {
                        fully_overlapping_assignment_count = fully_overlapping_assignment_count + 1;
                    }

                    if ranges_overlap_at_all( &range_bounds ) {
                        any_overlapping_assignment_count = any_overlapping_assignment_count + 1;
                    }                    
                }
            }
        }
    }

    println!("Overlapping count: {}", fully_overlapping_assignment_count );
    println!("Any Overlapping count: {}", any_overlapping_assignment_count );

}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
