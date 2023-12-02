use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(&input);

    let mut values: Vec<i32> = Vec::new();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
            
        if line.is_empty() {
            return;
        }

        values.push(find_numbers(&line));

        println!("Line: {}", values[0]);
    });

    print!("Sum of all calibration values: {}", values.iter().sum::<i32>());


}

fn find_numbers(line: &str) -> i32 {
    let mut first = 0;
    let mut last = 0;
    
    for character in line.chars() {
        if character.is_digit(10) {
            first = character.to_digit(10).unwrap() as usize;
            break;
        }
    }
    
    for character in line.chars().rev() {
        if character.is_digit(10) {
            last = character.to_digit(10).unwrap() as usize;
            break;
        }
    }

    let number = &line[first..last];
    number.parse::<i32>().unwrap()
}

