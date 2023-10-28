use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;

    /* --- part one --- */
    let reader1 = BufReader::new(file.try_clone()?);
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
    let reader2 = BufReader::new(file);
    let mut max_calories : Vec<i32> = Vec::new();
    let mut current_calories = 0;

    reader2.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {
            if line.is_empty() {
                update_max_calories(&mut max_calories, current_calories);
                current_calories = 0;
            } else {
                current_calories += line.trim().parse::<i32>().unwrap_or(0);
            }
        });
    println!(
        "The three elfs carrying the most calories are carrying {} calories!",
        max_calories.iter().sum::<i32>()
    );

    Ok(())
}

fn update_max_calories(max_calories: &mut Vec<i32>, current_calories: i32) {
    if current_calories > 0 {
        max_calories.push(current_calories);
        max_calories.sort_unstable_by(|a, b| b.cmp(a));
        if max_calories.len() > 3 {
            max_calories.pop();
        }
    }
}
