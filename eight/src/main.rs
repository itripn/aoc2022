use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calc_col_scenic_score( i : usize, j : usize, trees: &[Vec<u8>] ) -> (u32, u32) {

    let mut up_score : u32 = 0;
    let mut dn_score : u32 = 0;
    let target = trees[i][j];

    for it in (0..i).rev() {

        if trees[it][j] >= target {
            up_score += 1;
            break;
        }
        else if trees[it][j] < target {
            up_score += 1;
        }
    }

    for it in i+1..trees.len() {

        if trees[it][j] >= target {
            dn_score += 1;
            break;
        }
        else if trees[it][j] < target {
            dn_score += 1;
        }
    }

    (up_score, dn_score)

}

fn calc_row_scenic_score( i : usize, j : usize, trees: &[Vec<u8>] ) -> (u32,u32) {

    let row = &trees[i];
    let target = trees[i][j];
    let mut left_score : u32 = 0;
    let mut right_score : u32 = 0;

    
    for jt in (0..j).rev() {
        if row[jt] >= target {
            left_score += 1;
            break;
        }
        else if row[jt] <= target {
            left_score += 1;
        }
    }

    for jt in j+1..row.len() {

        if row[jt] >= target {
            right_score += 1;
            break;
        }
        else if row[jt] <= target {
            right_score += 1;
        }
    }

( left_score, right_score )

}



fn check_row( i : usize, j : usize, trees: &[Vec<u8>] ) -> bool {

    let row = &trees[i];
    let mut is_vis_from_left = true;
    let mut is_vis_from_right = true;
    let target = trees[i][j];

    // check from left
    //
    for t in row.iter().take(j) {

        if t >= &target {
            is_vis_from_left = false;
            break;
        }
    }

    // Check to end of row...
    // 
    for t in row.iter().skip(j+1) {

        if t >= &target {
            is_vis_from_right = false;
            break;
        }
    }

    // if is_vis_from_left || is_vis_from_right {
    //     println!("Tree is visible in it's row: {},{} ({})", i, j, target );
    // }

    is_vis_from_left || is_vis_from_right

}

fn check_col( i : usize, j : usize, trees: &[Vec<u8>] ) -> bool {

    let mut is_visible_up = true;
    let mut is_visible_dn = true;
    let target = trees[i][j];

    for tc in trees.iter().take(i) {

        if tc[j] >= target {
            is_visible_up = false;
            break;
        }
    }

    for tc in trees.iter().skip(i+1) {

        if tc[j] >= target {
            is_visible_dn = false;
            break;
        }
    }

    is_visible_dn || is_visible_up

}

fn calculate_best_scenic_score( trees : &[Vec<u8>] ) {

    let mut best_score = 0;

    for i in 1..trees.len() - 1 {
        for j in 1..trees[i].len() - 1 {

            let (ls, rs) = calc_row_scenic_score(i, j, trees);
            let (ts, ds) = calc_col_scenic_score(i, j, trees);

            // println!("{},{},{},{}", ls, rs, ts, ds);
            let temp_score = (ls*rs*ts*ds) as u32;
            
            if temp_score > best_score {
                best_score = temp_score;
            }
        }
    }

    println!("Best Scenic Score: {}", best_score );
}


fn count_visible_trees( trees : &[Vec<u8>] ) {

    let mut count = ((trees[0].len() * 2) + (trees.len() * 2 - 4)) as u32;

    for i in 1..trees.len() - 1 {
        for j in 1..trees[i].len() - 1 {

            let is_visible = check_row( i, j, trees ) || check_col( i, j, trees );
            if is_visible {
                count += 1;
            }
        }
    }

    println!("\nVisible trees: {}", count);

}

fn main() {
    let mut trees: Vec<Vec<u8>> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            if !line.is_empty() {
                trees.push(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect(),
                );
            }
        }
    }

    count_visible_trees(&trees);
    calculate_best_scenic_score(&trees);

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
