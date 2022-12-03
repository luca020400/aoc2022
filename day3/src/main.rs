#![warn(clippy::pedantic)]

use itertools::Itertools;
use std::{collections::HashSet, error::Error, fs};

fn calc_score(c: char) -> Result<u32, &'static str> {
    match c as u8 {
        b'A'..=b'Z' => Ok(u32::from(c as u8 - b'A' + 27)),
        b'a'..=b'z' => Ok(u32::from(c as u8 - b'a' + 1)),
        _ => Err("Unknown item"),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let part_one: u32 = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let len = line.len();
            let left: HashSet<_> = line[..len / 2].chars().collect();
            let right: HashSet<_> = line[len / 2..].chars().collect();
            let intersection: HashSet<_> = left.intersection(&right).collect();
            assert_eq!(1, intersection.len());
            match intersection.iter().at_most_one().unwrap() {
                Some(c) => calc_score(**c).unwrap(),
                None => panic!("Can't happen"),
            }
        })
        .sum();

    println!("Part one: {part_one}");

    let part_two: u32 = fs::read_to_string("input.txt")?
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(x, y, z)| {
            let xs: HashSet<_> = x.chars().collect();
            let ys: HashSet<_> = y.chars().collect();
            let zs: HashSet<_> = z.chars().collect();
            let xy: HashSet<_> = xs.intersection(&ys).copied().collect();
            let xyx: HashSet<_> = xy.intersection(&zs).collect();
            assert_eq!(1, xyx.len());
            match xyx.iter().at_most_one().unwrap() {
                Some(c) => calc_score(**c).unwrap(),
                None => panic!("Can't happen"),
            }
        })
        .sum();
    println!("Part two: {part_two}");

    Ok(())
}
