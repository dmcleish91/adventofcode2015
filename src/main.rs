use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let final_floor = input.chars().fold(0, |floor, c| {
        match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        }
    });

    println!("Final floor: {}", final_floor);

    let mut current_floor = 0;
    for (i, c) in input.chars().enumerate() {
        current_floor = match c {
            '(' => current_floor + 1,
            ')' => current_floor - 1,
            _ => current_floor,
        };

        if current_floor == -1 {
            println!("Position of first character that enters the basement: {}", i + 1);
            break;
        }
    }

    Ok(())
}
