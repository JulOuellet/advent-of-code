use std::{
    fs::File, 
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);
    
    let lines: Vec<String> = reader.lines().map(|line| line.expect("Error reading line")).collect();
    let mut stacks_p1 = init_stacks(&lines);
    let mut stacks_p2 = init_stacks(&lines);

    // Part one
    lines.iter().skip(9)
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let moves = get_moves(line);
            
            for _ in 0..moves.0 {
                if let Some(moved_crate) = stacks_p1[moves.1 - 1].pop() {
                    stacks_p1[moves.2 - 1].push(moved_crate);
                } else {
                    println!("Error moving crates around!");
                }
            }
        });

    println!(
        "The crates on top of each stacks with CrateMover 9000 are {}{}{}{}{}{}{}{}{}", 
        stacks_p1[0].last().unwrap(),
        stacks_p1[1].last().unwrap(),
        stacks_p1[2].last().unwrap(),
        stacks_p1[3].last().unwrap(),
        stacks_p1[4].last().unwrap(),
        stacks_p1[5].last().unwrap(),
        stacks_p1[6].last().unwrap(),
        stacks_p1[7].last().unwrap(),
        stacks_p1[8].last().unwrap(),
        );


    // Part two
    lines.iter().skip(9)
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let moves = get_moves(line);
            let mut lifted_crates: Vec<char> = Vec::new();
            
            for _ in 0..moves.0 {
                if let Some(moved_crate) = stacks_p2[moves.1 - 1].pop() {
                    lifted_crates.push(moved_crate);
                } else {
                    println!("Error lifting crates!")
                }
            }

            for _ in 0..moves.0 {
                stacks_p2[moves.2 - 1].push(lifted_crates.pop().unwrap()); 
            }
        });
    
    println!(
        "The crates on top of each stacks with CrateMover 9001 are {}{}{}{}{}{}{}{}{}", 
        stacks_p2[0].last().unwrap(),
        stacks_p2[1].last().unwrap(),
        stacks_p2[2].last().unwrap(),
        stacks_p2[3].last().unwrap(),
        stacks_p2[4].last().unwrap(),
        stacks_p2[5].last().unwrap(),
        stacks_p2[6].last().unwrap(),
        stacks_p2[7].last().unwrap(),
        stacks_p2[8].last().unwrap(),
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

