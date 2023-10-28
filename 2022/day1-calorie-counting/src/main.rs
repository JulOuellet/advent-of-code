use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    // part one 
    let mut max_calories = 0;
    let mut current_calories = 0;

    reader.lines()
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
    Ok(())
}

