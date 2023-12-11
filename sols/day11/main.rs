#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
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
    let input = read_file();
    let mut universe_org = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut universe = Vec::new();

    // Exapand the rows
    for row in universe_org {
        if row.iter().all(|&c| c == '.') {
            universe.push(row.clone());
        }

        universe.push(row.clone());
    }
    universe_org = transpose(&universe);
    universe.clear();

    // Expand the columns
    for col in universe_org {
        if col.iter().all(|&c| c == '.') {
            universe.push(col.clone());
        }

        universe.push(col.clone());
    }

    universe = transpose(&universe);

    let mut galaxies = Vec::new();

    universe.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &col)| {
            if col == '#' {
                galaxies.push((i, j));
            }
        });
    });

    // Since galaxies can be passed through then the shortest distance is just a straight line between them
    let mut total = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for (j, galaxy2) in galaxies.iter().skip(i + 1).enumerate() {
            let distance = (galaxy1.0 as i32 - galaxy2.0 as i32).abs() + (galaxy1.1 as i32 - galaxy2.1 as i32).abs();
            total += distance;
        }
    }

    println!("Part 1: {}", total);
}

fn part_two() {
    let input = read_file();
    let mut universe_org = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut universe = Vec::new();
    let step_size = 1000000;

    // Exapand the rows
    for row in universe_org {
        if row.iter().all(|&c| c == '.') {
            universe.push(vec!['x'; row.len()]); //x means step_size
        } else {
            universe.push(row.clone());
        }
    }
    universe_org = transpose(&universe);
    universe.clear();

    // Expand the columns
    for col in universe_org {
        if col.iter().all(|&c| c == '.' || c == 'x') {
            universe.push(vec!['x'; col.len()]);
        } else {
            universe.push(col.clone());
        }
    }

    universe = transpose(&universe);

    let mut galaxies = Vec::new();

    universe.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &col)| {
            if col == '#' {
                galaxies.push((i, j));
            }
        });
    });

    // Since galaxies can be passed through then the shortest distance is just a straight line between them
    let mut total: u64 = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for (j, galaxy2) in galaxies.iter().skip(i + 1).enumerate() {
            let mut steps = 0;
            let mut current_pos = (galaxy1.0, galaxy1.1);
            // Step vertically
            while current_pos.0 != galaxy2.0 {
                if universe[current_pos.0][current_pos.1] == 'x' {
                    steps += step_size;
                } else {
                    steps += 1;
                }

                if current_pos.0 < galaxy2.0 {
                    current_pos.0 += 1;
                } else {
                    current_pos.0 -= 1;
                }
            }

            // Step horizontally
            while current_pos.1 != galaxy2.1 {
                if universe[current_pos.0][current_pos.1] == 'x' {
                    steps += step_size;
                } else {
                    steps += 1;
                }

                if current_pos.1 < galaxy2.1 {
                    current_pos.1 += 1;
                } else {
                    current_pos.1 -= 1;
                }
            }

            total += steps;
        }
    }

    println!("Part 2: {}", total);
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
