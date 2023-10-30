use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(&file);
    let mut score = 0;

    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .for_each(|line| {

            // Last line is empty, we can print result
            if line.is_empty() {
                println!("Final score is: {}", score);
                return ()
            }

            let char_vec: Vec<char> = line.chars().collect();

            match char_vec[0] {
                'A' => {
                    match char_vec[2] {
                        'X' => score += 4, // 1 pt for choosing rock + 3 for draw
                        'Y' => score += 8, // 2 pts for choosing paper + 6 for win
                        'Z' => score += 3, // 3 pts for choosing scissors + 0 for loss
                        _ => println!("Unexpected character!")
                    }
                },
                'B' => {
                    match char_vec[2] {
                        'X' => score += 1, // 1 pt for choosing rock + 0 for loss
                        'Y' => score += 5, // 2 pts for choosing paper + 3 for draw
                        'Z' => score += 9, // 3 pts for choosing scissors + 6 for win
                        _ => println!("Unexpected character!")
                    }
                },
                'C' => {
                    match char_vec[2] {
                        'X' => score += 7, // 1 pt for choosing rock + 6 for win
                        'Y' => score += 2, // 2 pts for choosing paper + 0 for loss
                        'Z' => score += 6, // 3 pts for choosing scissors + 3 for draw
                        _ => println!("Unexpected character!")
                    }
                },
                _ => println!("Unexpected character!")
            }
        });

    Ok(())
}
