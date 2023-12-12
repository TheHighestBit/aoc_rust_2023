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
    let input = read_file();
    let mut rows = Vec::new();
    let mut broken = Vec::new();
    let mut total = 0;

    input.iter().for_each(|s| {
        let split = s.split(' ').collect::<Vec<&str>>();
        rows.push(split[0].chars().collect::<Vec<char>>());
        broken.push(split[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    });

    for i in 0..rows.len() {
        let result = find_ways(&mut rows[i], &broken[i], 0);
        total += result;
    }

    println!("Part 1: {}", total);
}

fn part_two() {
    let input = read_file();
    let mut rows = Vec::new();
    let mut broken = Vec::new();
    let mut total = 0;

    input.iter().for_each(|s| {
        let split = s.split(' ').collect::<Vec<&str>>();
        let mut result_row = Vec::new();
        let mut result_broken = Vec::new();
        let row_vec = split[0].chars().collect::<Vec<char>>();
        let broken_vec = split[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        result_row.extend(row_vec.clone());
        for _ in 0..4 {
            result_row.extend(vec!['?']);
            result_row.extend(row_vec.clone());
        }

        rows.push(result_row);

        for _ in 0..4 {
            result_broken.extend(broken_vec.clone());
        }

        broken.push(result_broken);
    });

    for i in 0..rows.len() {
        let result = find_ways(&mut rows[i], &broken[i], 0);
        total += result;
    }

    println!("Part 2: {}", total);
}

fn find_ways(row: &mut Vec<char>, broken: &[i32], index: usize) -> i32 {
    if index == row.len() {
        let mut broken_index = 0;
        let mut run_length = 0;

        for &mut c in row {
            if c == '#' {
                run_length += 1;
            } else if run_length > 0 {
                if broken_index >= broken.len() || broken[broken_index] != run_length {
                    return 0;
                }
                run_length = 0;
                broken_index += 1;
            }
        }

        // Check the last run
        if run_length > 0 {
            if broken_index >= broken.len() || broken[broken_index] != run_length {
                return 0;
            }
            run_length = 0;
            broken_index += 1;
        }

        return if broken_index == broken.len() { 1 } else { 0 };
    }

    let mut total = 0;

    if row[index] == '?' {
        // Try setting the current '?' to '.'
        row[index] = '.';
        total += find_ways(row, broken, index + 1);

        // Try setting the current '?' to '#'
        row[index] = '#';
        total += find_ways(row, broken, index + 1);

        // Reset the character back to '?'
        row[index] = '?';
    } else {
        total += find_ways(row, broken, index + 1);
    }

    total
}