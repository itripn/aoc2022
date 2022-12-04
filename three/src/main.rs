
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn part_one( sack_lines : &Vec<String> ) {

    let mut total_priority : u32 = 0;

    sack_lines
    .iter()
    .for_each(|sack| {

        let c1 : String = sack.to_string()[0..sack.len()/2].to_string();
        let c2 : String = sack.to_string()[sack.len()/2..].to_string();
        let c1set : HashSet<char> = c1.chars().collect();
        let c2set : HashSet<char> = c2.chars().collect();

        let c = c1set.intersection(&c2set).next().unwrap();        
        if c.is_ascii_lowercase() { 
            total_priority = total_priority + ( ((*c as u8) - ('a' as u8) + 1) as u32);
        }
        else {
            total_priority = total_priority + (((*c as u8) - ('A' as u8) + 27) as u32);
        }
    });

    println!("Total Priority: {}", total_priority );

}


fn part_two( sack_lines : &Vec<String> ) {

    let mut group : Vec<&String> = Vec::new();
    let mut total_priority : u32 = 0;

    sack_lines
    .iter()
    .for_each( |sack| {

        if group.len() != 3 {
            group.push( sack );
        }
        else {
            total_priority = total_priority + calc_priority(&group);            
            group.clear();
            group.push(sack);
        }        

    });

    total_priority = total_priority + calc_priority(&group);

    println!("Total priority: {}", total_priority );

}

fn calc_priority(group: &Vec<&String>) -> u32 {
   
    let mut pri = 0;
    
    assert!( group.len() == 3, "Wrong number of sacks for group");
    let s1 : HashSet<char> = group[0].chars().collect();
    let s2 : HashSet<char> = group[1].chars().collect();
    let s3 : HashSet<char> = group[2].chars().collect();
    let i1 : HashSet<char> = s1.intersection(&s2).cloned().collect();

    let mut i2  = i1.intersection(&s3);
    let c = i2.next().unwrap();
    if c.is_ascii_lowercase() { 
        pri = pri + ( ((*c as u8) - ('a' as u8) + 1) as u32);
    }
    else {
        pri = pri + (((*c as u8) - ('A' as u8) + 27) as u32);
    }

    pri
}


fn main() {

    let mut sack_lines : Vec<String> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {
                    sack_lines.push( l );
                }
            }
        }
    }
    

    part_one( &sack_lines );
    part_two( &sack_lines );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>( filename: P ) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open( filename )?;
    Ok( io::BufReader::new( file ).lines() )
}
