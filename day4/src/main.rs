#![warn(clippy::pedantic)]

use std::ops::RangeInclusive;
use std::{error::Error, fs};

trait Overlap<U = Self> {
    fn overlaps(&self, other: U) -> bool;
}

impl<T: std::cmp::PartialOrd> Overlap for RangeInclusive<T> {
    fn overlaps(&self, other: Self) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (first, second) = match line.split_once(',') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;

            let split_into_range = |str: &str| -> Result<RangeInclusive<u32>, Box<dyn Error>> {
                let (left, right) = match str.split_once('-') {
                    Some(x) => Ok(x),
                    None => Err("Can't split the string"),
                }?;
                let left = left.parse::<u32>()?;
                let right = right.parse::<u32>()?;
                Ok(left..=right)
            };
            let first = split_into_range(first)?;
            let second = split_into_range(second)?;

            Ok(u32::from(
                first.contains(second.start()) && first.contains(second.end())
                    || second.contains(first.start()) && second.contains(first.end()),
            ))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let (first, second) = match line.split_once(',') {
                Some(x) => Ok(x),
                None => Err("Can't split the string"),
            }?;

            let split_into_range = |str: &str| -> Result<RangeInclusive<u32>, Box<dyn Error>> {
                let (left, right) = match str.split_once('-') {
                    Some(x) => Ok(x),
                    None => Err("Can't split the string"),
                }?;
                let left = left.parse::<u32>()?;
                let right = right.parse::<u32>()?;
                Ok(left..=right)
            };
            let first = split_into_range(first)?;
            let second = split_into_range(second)?;

            Ok(u32::from(first.overlaps(second)))
        })
        .sum::<Result<u32, _>>()?;

    println!("Part two: {part_two}");

    Ok(())
}
