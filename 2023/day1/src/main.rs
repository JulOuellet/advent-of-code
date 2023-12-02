use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(&input);

    let mut values: Vec<i32> = Vec::new();

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .for_each(|line| values.push(find_numbers(&line)));

    println!("Sum of all calibration values: {}", values.iter().sum::<i32>());

    Ok(())
}

fn find_numbers(line: &str) -> i32 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

    let combined = format!("{}{}", first, last);
    combined.parse::<i32>().unwrap_or(0)
}

