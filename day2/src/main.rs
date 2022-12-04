#![warn(clippy::pedantic)]

use std::{error::Error, fs, u8};

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSS: u8 = 0;

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (a, b) = match line.split_once(' ') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;
            let mut a = a.parse::<char>()? as u8;
            let mut x = b.parse::<char>()? as u8;

            // Map moves to scores
            a -= b'A';
            x -= b'X';

            // Count score of 2nd player
            // Ensuring we don't overflow
            Ok(u32::from(
                x + 1
                    + match (x + 3 - a).rem_euclid(3) {
                        1 => WIN,
                        0 => DRAW,
                        _ => LOSS,
                    },
            ))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (a, b) = match line.split_once(' ') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;
            let mut a = a.parse::<char>()? as u8;
            let x = b.parse::<char>()? as u8;

            // Map move to score
            a -= b'A';

            // Ensure we don't overflow
            Ok(u32::from(match x {
                // Loss
                b'X' => (a + 3 - 1).rem_euclid(3) + 1 + LOSS,
                // Draw
                b'Y' => (a).rem_euclid(3) + 1 + DRAW,
                // Win
                b'Z' => (a + 1).rem_euclid(3) + 1 + WIN,
                _ => panic!("Unknown win situation"),
            }))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part two: {part_two}");

    Ok(())
}
