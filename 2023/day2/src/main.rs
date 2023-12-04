use std::{
    fs::File,
    io::{BufReader, BufRead}
};

fn main() -> std::io::Result<()> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(&input);

    let mut possible_game_ids = 0;
    
    reader.lines()
        .map(|line| line.expect("Error reading line..."))
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(index, line)| {
            if is_possilbe(&line) {
                possible_game_ids += index + 1;
            }
        });

    println!("Possible game ids: {}", possible_game_ids);

    Ok(())
}


fn is_possilbe(game: &str) -> bool {
    let mut is_possible = true;

    let (_, color_data) = game.split_once(": ").unwrap();

    color_data.split(';').for_each(|segment| {
        let mut blue_count = 0;
        let mut red_count = 0;
        let mut green_count = 0;

        segment.split(',').for_each(|color_count| {
            let parts: Vec<&str> = color_count.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let count: i32 = parts[0].parse().unwrap_or(0);
                match parts[1] {
                    "blue" => blue_count += count,
                    "red" => red_count += count,
                    "green" => green_count += count,
                    _ => {}
                }
            }
        });

        if red_count > 12 || green_count > 13 || blue_count > 14 {
            is_possible = false;
        }
    });
    
    is_possible
}
