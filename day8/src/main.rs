#![warn(clippy::pedantic)]

use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let grid: Vec<&str> = input.lines().collect();
    let mut part_one = 0;

    grid.iter().enumerate().for_each(|(i, row)| {
        row.bytes().enumerate().for_each(|(j, tree)| {
            let mut left = 1;
            for jj in 0..j {
                left &= u32::from(row.bytes().nth(jj).unwrap() < tree);
            }
            let mut right = 1;
            for jj in j + 1..row.len() {
                right &= u32::from(row.bytes().nth(jj).unwrap() < tree);
            }
            let mut up = 1;
            for ii in 0..i {
                up &= u32::from(grid[ii].bytes().nth(j).unwrap() < tree);
            }
            let mut down = 1;
            for ii in i + 1..grid.len() {
                down &= u32::from(grid[ii].bytes().nth(j).unwrap() < tree);
            }
            part_one += left | right | up | down;
        });
    });

    println!("Part one {part_one}");

    let mut part_two_scores: Vec<u32> = Vec::new();

    grid.iter().enumerate().for_each(|(i, row)| {
        row.bytes().enumerate().for_each(|(j, tree)| {
            let mut left = 0;
            for jj in (0..j).into_iter().rev() {
                let tree_cmp = row.bytes().nth(jj).unwrap();
                left = left + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut right = 0;
            for jj in j + 1..row.len() {
                let tree_cmp = row.bytes().nth(jj).unwrap();
                right = right + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut up = 0;
            for ii in (0..i).into_iter().rev() {
                let tree_cmp = grid[ii].bytes().nth(j).unwrap();
                up = up + 1;
                if tree_cmp >= tree {
                    break;
                }
            }
            let mut down = 0;
            for ii in i + 1..grid.len() {
                let tree_cmp = grid[ii].bytes().nth(j).unwrap();
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
