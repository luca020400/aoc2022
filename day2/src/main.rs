#![warn(clippy::pedantic)]

use std::{error::Error, fs, u8};

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let mut a = a.parse::<char>().unwrap() as u8;
            let mut x = b.parse::<char>().unwrap() as u8;

            // Map moves to scores
            a -= b'A';
            x -= b'X';

            // Count score of 2nd player
            (u32::from(x) + 1)
                + match (x as i8 - a as i8).rem_euclid(3) {
                    // Win
                    1 => 6,
                    // Draw
                    0 => 3,
                    // Loss
                    _ => 0, // Can only be 2
                }
        })
        .sum();

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let mut a = a.parse::<char>().unwrap() as u8;
            let x = b.parse::<char>().unwrap() as u8;

            // Map move to score
            a -= b'A';

            match x {
                // Loss
                b'X' => {
                    let mut score = (a as i8 - 1).rem_euclid(3) + 1;
                    score += 0;
                    score as u32
                }
                // Draw
                b'Y' => {
                    let mut score = a as i8 + 1;
                    score += 3;
                    score as u32
                }
                // Win
                b'Z' => {
                    let mut score = (a as i8 + 1).rem_euclid(3) + 1;
                    score += 6;
                    score as u32
                }
                _ => panic!("Unknown win situation"),
            }
        })
        .sum();

    println!("Part two: {part_two}");

    Ok(())
}
