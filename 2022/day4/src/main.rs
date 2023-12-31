use std::{
    fs::File, 
    io::{BufRead, BufReader}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    let reader = BufReader::new(&file);
    let mut score_contain = 0;
    let mut score_overlap = 0;

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {

            if line.is_empty() {
                println!("There are {} pairs where one fully contains the other!", score_contain);
                println!("There are {} pairs where one overlaps with the other!", score_overlap);
            } else {
                let ranges: Vec<&str> = line.split(',').collect();

                if is_contained(&ranges) {
                    score_contain += 1;
                }
                if is_overlapped(&ranges) {
                    score_overlap += 1;
                }
            }
        });

    Ok(())
}

fn is_contained(ranges: &Vec<&str>) -> bool {
    let first_bounds: Vec<i32> = ranges[0]
        .split('-')
        .map(|bound| bound.trim().parse::<i32>().unwrap_or(0))
        .collect();
    
    let second_bounds: Vec<i32> = ranges[1]
        .split('-')
        .map(|bound| bound.trim().parse::<i32>().unwrap_or(0))
        .collect();
    
    if first_bounds[0] <= second_bounds[0] && first_bounds[1] >= second_bounds[1] {
        return true;
    } else if second_bounds[0] <= first_bounds[0] && second_bounds[1] >= first_bounds[1] {
        return true;
    }
    false
}

fn is_overlapped(ranges: &Vec<&str>) -> bool {
    let first_bounds: Vec<i32> = ranges[0]
        .split('-')
        .map(|bound| bound.trim().parse::<i32>().unwrap_or(0))
        .collect();
    
    let second_bounds: Vec<i32> = ranges[1]
        .split('-')
        .map(|bound| bound.trim().parse::<i32>().unwrap_or(0))
        .collect();

    if first_bounds[0] <= second_bounds[1] && first_bounds[1] >= second_bounds[0] {
        return true;
    }
    false
}

