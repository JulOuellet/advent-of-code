use std::{
    fs::File,
    io::{BufReader, BufRead, Seek, SeekFrom},
    collections::{HashMap, HashSet}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let priorities = generate_priorities();

    /* --- part one --- */
    let reader1 = BufReader::new(&file);
    let mut total_score_p1 = 0;

    reader1.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| total_score_p1 += get_score_p1(&line, &priorities));
    println!("Sum of priorities for part one: {}", total_score_p1);

    /* --- part two ---*/
    let mut reader2 = BufReader::new(&file);
    let _ = reader2.get_mut().seek(SeekFrom::Start(0));
    let mut total_score_p2 = 0;

    Ok(())
}

fn generate_priorities() -> HashMap<char, i32> {
    ('a'..='z')
        .enumerate()
        .map(|(index, char)| (char, index as i32 + 1))
        .chain(('A'..='Z')
               .enumerate()
               .map(|(index, char)| (char, index as i32 + 27)))
        .collect()
}

fn get_score_p1(line: &str, priorities: &HashMap<char, i32>) -> i32 {
    let mut score = 0;

    let middle_idx = line.len() / 2;
    let first_comp: HashSet<_> = line[..middle_idx].chars().collect();
    let second_comp: HashSet<_> = line[middle_idx..].chars().collect();
    
    let reccuring_items: Vec<char> = first_comp.intersection(&second_comp).copied().collect();
    reccuring_items.iter().for_each(|item| score += priorities.get(item).unwrap_or(&0));

    score
}

