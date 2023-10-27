use std::{
    fs::File, 
    io::{BufRead, BufReader}
};

fn main() -> std::io::Result<()> {

    let mut max_calories = 0;
    let mut current_calories = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
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

    println!("{}", max_calories);

    Ok(())
}

