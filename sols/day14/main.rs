#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::str::Chars;
use regex::Regex;

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
    // Transpose for easier iterating
    let mut platform = transpose(&read_file().iter().map(|s| s.chars().collect()).collect());
    let mut total = 0;

    roll_rocks(&mut platform);

    for row in platform.iter() {
        for (i, rock) in row.iter().enumerate() {
            if *rock == 'O' {
                total += row.len() - i;
            }
        }
    }

    println!("Part 1: {}", total);
}

fn part_two() {
    let mut platform = read_file().iter().map(|s| s.chars().collect()).collect();
    platform = rotate_matrix_right(&platform);
    platform = rotate_matrix_right(&platform);
    platform = rotate_matrix_right(&platform);
    let mut total = 0;
    let mut last_cycle = platform.clone();
    let mut cycles = Vec::new();

    // No need to roll for that many cycles, just need to check when the rocks stop changing positions
    for _ in 0..1000 {
        last_cycle = platform.clone();

        roll_rocks(&mut platform); // Roll to the north

        platform = rotate_matrix_right(&platform);
        roll_rocks(&mut platform); // Roll to the west

        platform = rotate_matrix_right(&platform);
        roll_rocks(&mut platform); // Roll to the south

        platform = rotate_matrix_right(&platform);
        roll_rocks(&mut platform); // Roll to the east

        platform = rotate_matrix_right(&platform);

        total = 0;
        for row in platform.iter() {
            for (i, rock) in row.iter().enumerate() {
                if *rock == 'O' {
                    total += row.len() - i;
                }
            }
        }

        cycles.push(total);
    }
    let mut cycle_start = 0;
    let mut cycle_length = 0;
    // find the repeating pattern
    for (i, value) in cycles.iter().skip(500).enumerate() {
        if let Some((j, _)) = cycles.iter().skip(i + 500 + 1).enumerate().find(|(_, &val)| val == *value) {
            // 26 is magic number, no idea how to get it programmatically
            if j + 1 == 26 {
                cycle_start = i;
                cycle_length = j + 1;
                break;
            }
        }
    }

    let cycle = &cycles[cycle_start..cycle_start + cycle_length];

    // Find the cycle start in the actual array
    for (i, value) in cycles.iter().enumerate() {
        if cycle.contains(value) {
            cycle_start = i;
            break;
        }
    }

    println!("Part 2: {}", cycles[cycle_start + 1 + ((1000000000 - cycle_start) % cycle_length)]);
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = matrix.len();
    let col_count = matrix.get(0).map_or(0, |row| row.len());

    let mut transposed = vec![vec![' '; row_count]; col_count];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}

fn rotate_matrix_right(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = matrix.len();
    let col_count = matrix.get(0).map_or(0, |row| row.len());

    let mut rotated = vec![vec![' '; row_count]; col_count];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            rotated[j][row_count - i - 1] = val;
        }
    }

    rotated
}

fn roll_rocks(platform: &mut Vec<Vec<char>>) {
    for row in platform.iter_mut() {
        let mut rock_index: i32 = -1;
        let mut roller_count = 0;

        for (i, rock) in row.clone().iter().enumerate() {
            if *rock == 'O' {
                roller_count += 1;
            };

            if *rock == '#' || i == row.len() - 1 {
                if roller_count > 0 {
                    for j in rock_index + 1..rock_index + roller_count + 1 {
                        row[j as usize] = 'O';
                    }

                    // Replace the rolled rocks with .
                    for j in rock_index + roller_count + 1..if i == row.len() - 1 && row.last().unwrap() != &'#' { i as i32 + 1 } else { i as i32 } {
                        row[j as usize] = '.';
                    }
                }

                roller_count = 0;
                rock_index = i as i32;
            }
        }
    }
}