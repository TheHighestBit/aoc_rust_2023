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
    let mut matrices = Vec::new();
    let mut matrix = Vec::new();
    let mut total = 0;

    for line in read_file() {
        if line.is_empty() {
            matrices.push(matrix);
            matrix = Vec::new();
        } else {
            matrix.push(line.chars().collect::<Vec<char>>());
        }
    }
    matrices.push(matrix);

    // Very small input, just check all possible reflections
    for matrix in matrices {
        // Check column wise first
        let transposed = transpose(&matrix);
        let vertical = get_reflection_index_a(&transposed);

        if vertical == 0 { // Also need to check horizontal
            let horizontal = get_reflection_index_a(&matrix);
            total += horizontal * 100;
        } else {
            total += vertical;
        }
    }

    println!("Part 1: {}", total);
}

fn part_two() {
    let mut matrices = Vec::new();
    let mut matrix = Vec::new();
    let mut total = 0;

    for line in read_file() {
        if line.is_empty() {
            matrices.push(matrix);
            matrix = Vec::new();
        } else {
            matrix.push(line.chars().collect::<Vec<char>>());
        }
    }
    matrices.push(matrix);

    // Very small input, just check all possible reflections
    for matrix in matrices {
        // Check column wise first
        let transposed = transpose(&matrix);
        let vertical = get_reflection_index_b(&transposed);

        if vertical == 0 { // Also need to check horizontal
            let horizontal = get_reflection_index_b(&matrix);
            total += horizontal * 100;
        } else {
            total += vertical;
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

fn get_reflection_index_a(matrix: &[Vec<char>]) -> usize {
    for i in 1..matrix.len() {
        let mut is_reflection = true;

        for j in 1..=i {
            if i + j > matrix.len() {
                break;
            }
            if matrix[i - j] != matrix[i + j - 1] {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            return i;
        }
    }

    0
}

fn get_reflection_index_b(matrix: &[Vec<char>]) -> usize {
    for i in 1..matrix.len() {
        let mut is_smudged = false;
        let mut is_reflection = true;

        for j in 1..=i {
            if i + j > matrix.len() {
                break;
            }
            if matrix[i - j] != matrix[i + j - 1] {
                if !is_smudged && is_one_off(&matrix[i - j], &matrix[i + j - 1]) {
                    is_smudged =  true;
                } else {
                    is_reflection = false;
                    break;
                }
            }
        }

        if is_reflection && is_smudged {
            return i;
        }
    }

    0
}

fn is_one_off(row1: &[char], row2: &[char]) -> bool {
    let mut one_off = false;

    for i in 0..row1.len() {
        if row1[i] != row2[i] {
            if one_off {
                return false;
            }
            one_off = true;
        }
    }

    one_off
}