use std::{
    fs::File, 
    io::{BufReader, BufRead, SeekFrom, Seek}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    /* --- part one --- */
    let reader1 = BufReader::new(&file);
    let mut max_calories = 0;
    let mut current_calories = 0;

    reader1.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {
            
            // If line is empty, we have a new elf
            if line.is_empty() {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            } else {
                current_calories += line.trim().parse::<i32>().unwrap_or(0);
            }
        });
    println!("The elf carrying the most calories is carrying {} calories!", max_calories);

    /* --- part two --- */
    let mut reader2 = BufReader::new(&file);
    let _ = reader2.get_mut().seek(SeekFrom::Start(0));

    let mut current_calories = 0;
    let mut top_three: Vec<i32> = vec![0, 0, 0];
    
    reader2.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {
            
            if line.is_empty() {
                 check_top_three(current_calories, &mut top_three);
                 current_calories = 0;
            } else {
                current_calories += line.trim().parse::<i32>().unwrap_or(0);
            }
        });
    println!("The three elves carrying the most calories are carrying {} calories!",
             top_three.iter().sum::<i32>());

    Ok(())
}

fn check_top_three(current_calories: i32, top_three_vec: &mut Vec<i32>) {
    if current_calories > top_three_vec[0] {
        top_three_vec[0] = current_calories;
        top_three_vec.sort_unstable();
    }
}

