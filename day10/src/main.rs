#![warn(clippy::pedantic)]

use std::{error::Error, fs};

type REG = i32;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let ops: Vec<&str> = input.lines().collect();

    let mut x: REG = 1;
    let mut cycle: u32 = 0;

    let mut signals: Vec<i32> = Vec::new();
    let mut image: Vec<char> = Vec::new();
    image.push('\n');

    ops.iter().for_each(|op| {
        let mut tick = || {
            // Part two
            let range = x - 1..=x + 1;
            if range.contains(&(cycle as i32 % 40)) {
                image.push('#');
            } else {
                image.push('.');
            }
            // Common
            cycle += 1;
            // Part one
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => signals.push(cycle as i32 * x),
                _ => {}
            }
            // Part two
            if cycle % 40 == 0 {
                image.push('\n');
            }
        };

        let split: Vec<&str> = op.split(' ').collect();
        match split.len() {
            1 => {
                if split[0] == "noop" {
                    tick();
                }
            }
            2 => {
                if split[0] == "addx" {
                    tick();
                    tick();
                    x += split[1].parse::<i32>().unwrap();
                }
            }
            _ => {}
        };
    });

    let part_one: i32 = signals.iter().sum();
    let part_two: String = image.iter().collect();

    println!("Part one {}", part_one);

    println!("Part two {}", part_two);

    Ok(())
}
