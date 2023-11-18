use std::{
    fs::File, 
    io::{BufReader, BufRead}, 
    collections::VecDeque
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);

    let mut char_sequence: VecDeque<char> = VecDeque::new();
    let mut index = 0;

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            for character in line.chars() {
                char_sequence.push_back(character);
                index += 1;
            
                if char_sequence.len() > 4 {
                    char_sequence.pop_front();

                    if check_squence(&char_sequence) {
                        println!("First marker detected after character {}", index);
                        break;
                    }
                }
            }
        });
    Ok(())
}

fn check_squence(char_sequence: &VecDeque<char>) -> bool {
    let mut chars: Vec<char> = Vec::new();

    for index in 0..4 {
        if let Some(&character) = char_sequence.get(index) {
            if chars.contains(&character) {
                return false;
            }
            chars.push(character);
        }
    }
    true
}

