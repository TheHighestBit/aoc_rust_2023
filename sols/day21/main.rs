#![allow(unused)]

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    part_one();
    part_two();
}

fn read_file() -> Vec<String> {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn part_one() {
    let input = read_file();
    let grid = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut current_positions = HashSet::new();

    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (x, y);
            }
        }
    };

    let mut new_positions = HashSet::new();
    current_positions.insert(start);

    for _ in 0..64 {
        for pos in current_positions.iter() {
            // Up
            if pos.1 > 0 && grid[pos.1 - 1][pos.0] != '#' {
                new_positions.insert((pos.0, pos.1 - 1));
            }

            // Right
            if pos.0 < grid[0].len() - 1 && grid[pos.1][pos.0 + 1] != '#' {
                new_positions.insert((pos.0 + 1, pos.1));
            }

            // Down
            if pos.1 < grid.len() - 1 && grid[pos.1 + 1][pos.0] != '#' {
                new_positions.insert((pos.0, pos.1 + 1));
            }

            // Left
            if pos.0 > 0 && grid[pos.1][pos.0 - 1] != '#' {
                new_positions.insert((pos.0 - 1, pos.1));
            }
        }

        current_positions = new_positions.clone();
        new_positions.clear();
    }

    println!("Part one: {}", current_positions.len());
}

fn part_two() {
    
}
