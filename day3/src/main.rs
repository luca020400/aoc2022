#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::{collections::HashSet, error::Error, fs};

fn calc_score(c: char) -> Result<u32, Box<dyn Error>> {
    match c as u8 {
        b'A'..=b'Z' => Ok(u32::from(c as u8 - b'A' + 27)),
        b'a'..=b'z' => Ok(u32::from(c as u8 - b'a' + 1)),
        _ => Err("Unknown item".into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| -> Result<u32, Box<dyn Error>> {
            let len = line.len();
            let left: HashSet<_> = line[..len / 2].chars().collect();
            let right: HashSet<_> = line[len / 2..].chars().collect();
            let intersection: HashSet<_> = left.intersection(&right).collect();
            match intersection.iter().exactly_one() {
                Ok(item) => calc_score(**item),
                _ => Err("More than one element".into()),
            }
        })
        .sum::<Result<u32, _>>()?;

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(x, y, z)| -> Result<u32, Box<dyn Error>> {
            let xs: HashSet<_> = x.chars().collect();
            let ys: HashSet<_> = y.chars().collect();
            let zs: HashSet<_> = z.chars().collect();
            let xy: HashSet<_> = xs.intersection(&ys).copied().collect();
            let xyx: HashSet<_> = xy.intersection(&zs).collect();
            match xyx.iter().exactly_one() {
                Ok(item) => calc_score(**item),
                _ => Err("More than one element".into()),
            }
        })
        .sum::<Result<u32, _>>()?;

    println!("Part two: {part_two}");

    Ok(())
}
