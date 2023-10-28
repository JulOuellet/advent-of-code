use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    part_one_solution(reader)
}

fn part_one_solution(reader: BufReader<File>) -> std::io::Result<()> {
    let mut max_calories = 0;
    let mut current_calories = 0;

    // iterate on lines
    for line in reader.lines() {
        let line = line?;

        // if current line is empty, we are done with the current elf
        if line.is_empty() {

            // if current calories is bigger than previous max, it becomes the new max
            if current_calories > max_calories {
                max_calories = current_calories;
                current_calories = 0;
            } else {
                current_calories = 0;
            }
        } else {
            current_calories += line.trim().parse().unwrap_or(0);
        }
    }

    println!("Total calories carried by the elf carrying the most calories: {}", max_calories);
    Ok(())
}

fn part_two_solution(reader: BufReader<File>) -> std::io::Result<()> {
    let mut most_calories = 0;
    let mut second_most_calories = 0;
    let mut third_most_calories = 0;
    let mut current_calories = 0;
    
    // iterate over lines

    Ok(()) 
}
