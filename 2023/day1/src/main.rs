use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(&input);

    let mut values_p1: Vec<i32> = Vec::new();
    let mut values_p2: Vec<i32> = Vec::new();

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .for_each(|line| {       
            values_p1.push(find_numbers_p1(&line));
            values_p2.push(find_numbers_p2(&line));
    });

    println!("Sum of all calibration values for part one: {}", values_p1.iter().sum::<i32>());
    println!("Sum of all calibration values for part two: {}", values_p2.iter().sum::<i32>());

    Ok(())
}

fn find_numbers_p1(line: &str) -> i32 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();

    10 * first.to_digit(10).unwrap() as i32 + last.to_digit(10).unwrap() as i32
}
    
fn find_numbers_p2(line: &str) -> i32 {
    let nums = &[
        "one", "1",
        "two", "2",
        "three", "3",
        "four", "4",
        "five", "5",
        "six", "6",
        "seven", "7",
        "eight", "8",
        "nine", "9",
    ];

    let ac = aho_corasick::AhoCorasick::new(nums).unwrap();

    let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
    let first = matches.first().unwrap().pattern().as_usize() / 2 + 1;
    let last = matches.last().unwrap().pattern().as_usize() / 2 + 1;

    10 * first as i32 + last as i32
}

