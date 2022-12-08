#![warn(clippy::pedantic)]

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let grid: Vec<&str> = input.lines().collect();
    let mut part_one = 0;

    grid.iter().enumerate().for_each(|(ext_i, row)| {
        row.chars().enumerate().for_each(|(ext_j, tree)| {
            let tree = tree as u8;
            let mut left = 1;
            for j in 0..ext_j {
                left &= u32::from(row.bytes().nth(j).unwrap() < tree);
            }
            let mut right = 1;
            for j in ext_j + 1..row.len() {
                right &= u32::from(row.bytes().nth(j).unwrap() < tree);
            }
            let mut up = 1;
            for i in 0..ext_i {
                up &= u32::from(grid[i].bytes().nth(ext_j).unwrap() < tree);
            }
            let mut down = 1;
            for i in ext_i + 1..grid.len() {
                down &= u32::from(grid[i].bytes().nth(ext_j).unwrap() < tree);
            }
            part_one += left | right | up | down;
        });
    });

    println!("Part one {part_one}");

    let mut part_two_scores: Vec<u32> = Vec::new();

    grid.iter().enumerate().for_each(|(ext_i, row)| {
        row.chars().enumerate().for_each(|(ext_j, tree)| {
            let tree = tree as u8;
            let mut left = 0;
            for j in (0..ext_j).into_iter().rev() {
                let tree_cmp = row.bytes().nth(j).unwrap();
                left = left + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut right = 0;
            for j in ext_j + 1..row.len() {
                let tree_cmp = row.bytes().nth(j).unwrap();
                right = right + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut up = 0;
            for i in (0..ext_i).into_iter().rev() {
                let tree_cmp = grid[i].bytes().nth(ext_j).unwrap();
                up = up + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut down = 0;
            for i in ext_i + 1..grid.len() {
                let tree_cmp = grid[i].bytes().nth(ext_j).unwrap();
                down = down + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            part_two_scores.push(left * right * up * down);
        });
    });

    let part_two = part_two_scores.iter().max();

    println!("Part two {:#?}", part_two);

    Ok(())
}
