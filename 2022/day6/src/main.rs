use std::{
    fs::File, 
    io::{BufReader, BufRead}, 
    collections::VecDeque
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);

    let mut packet_sequence: VecDeque<char> = VecDeque::new();
    let mut message_sequence: VecDeque<char> = VecDeque::new();

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut packet_index = 0;
            let mut message_index = 0;
            
            for character in line.chars() {
                packet_sequence.push_back(character);
                packet_index += 1;
            
                if packet_sequence.len() > 4 {
                    packet_sequence.pop_front();

                    if check_sequence(&packet_sequence, 4) {
                        println!("First start-of-packet marker detected after character {}", packet_index);
                        break;
                    }
                }
            }

            for character in line.chars() {
                message_sequence.push_back(character);
                message_index += 1;

                if message_sequence.len() > 14 {
                    message_sequence.pop_front();

                    if check_sequence(&message_sequence, 14) {
                        println!("First start-of-message marker detected after character {}", message_index);
                        break;
                    }
                }
            }
        });
    Ok(())
}

fn check_sequence(char_sequence: &VecDeque<char>, len: usize) -> bool {
    let mut chars: Vec<char> = Vec::new();

    for index in 0..len {
        if let Some(&character) = char_sequence.get(index) {
            if chars.contains(&character) {
                return false;
            }
            chars.push(character);
        }
    }
    true
}

