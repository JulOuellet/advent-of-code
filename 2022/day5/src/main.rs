use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);
    
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Error reading line")).collect();
    let stacks = init_stacks(&lines);
    
    Ok(())
}

fn init_stacks(lines: &[String]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];
    
    lines.iter().take(8).for_each(|line| {
        for (index, character) in line.chars().enumerate() {
            match index {
                1 | 5 | 9 | 13 | 17 | 21 | 25 | 29 | 33 => {
                    if character != ' ' {
                        let stack_index = index / 4;
                        stacks[stack_index].push(character);
                    }
                }
                _ => continue
            }
        }
    });

    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}

