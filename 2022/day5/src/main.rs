use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);
    
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Error reading line")).collect();
    let mut stacks = init_stacks(&lines);
    
    lines.iter().skip(9)
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let moves = get_moves(line);
            
            for _ in 0..moves.0 {
                if let Some(moved_crate) = stacks[moves.1 - 1].pop() {
                    stacks[moves.2 - 1].push(moved_crate);
                } else {
                    println!("Error moving crates around!");
                }
            }
        });

    println!(
        "The crates on top of each stacks are {:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}", 
        stacks[0].last(),
        stacks[1].last(),
        stacks[2].last(),
        stacks[3].last(),
        stacks[4].last(),
        stacks[5].last(),
        stacks[6].last(),
        stacks[7].last(),
        stacks[8].last(),
        );
    
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

fn get_moves(line: &String) -> (usize, usize, usize) {
    let moves: Vec<i32> = line
        .replace("move ", "")
        .replace(" from ", "-")
        .replace(" to ", "-")
        .split('-')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    (moves[0] as usize, moves[1] as usize, moves[2] as usize)
}

