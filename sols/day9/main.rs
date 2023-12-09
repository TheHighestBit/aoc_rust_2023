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
    let histories = read_file().iter()
    .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    .collect::<Vec<Vec<i32>>>();

    let mut result = 0;

    for history in histories {
        let mut differences = Vec::new();
        let mut next_line = history.clone();

        while !next_line.iter().all(|x| *x == 0) {
            let next_line_copy = next_line.clone();
            next_line.clear();

            for window in next_line_copy.windows(2) {
                next_line.push(window[1] - window[0]);
            }

            differences.push(next_line.clone());
        }

        differences.last_mut().unwrap().push(0);

        for i in (0..differences.len() - 2).rev() {
            let value_to_push = differences[i].last().unwrap() + differences[i + 1].last().unwrap();
            differences[i].push(value_to_push);
        }

        result += history.last().unwrap() + differences[0].last().unwrap();
    }

    println!("Part 1: {}", result);
}

fn part_two() {
    let histories = read_file().iter()
    .map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    .collect::<Vec<Vec<i32>>>();

    let mut result = 0;

    for history in histories {
        let mut differences = Vec::new();
        let mut next_line = history.clone();

        while !next_line.iter().all(|x| *x == 0) {
            let next_line_copy = next_line.clone();
            next_line.clear();

            for window in next_line_copy.windows(2) {
                next_line.push(window[1] - window[0]);
            }

            differences.push(next_line.clone());
        }

        differences.last_mut().unwrap().insert(0, 0);

        for i in (0..differences.len() - 2).rev() {
            let value_to_insert = differences[i].first().unwrap() - differences[i + 1].first().unwrap();
            differences[i].insert(0, value_to_insert);
        }

        result += history.first().unwrap() - differences[0].first().unwrap();
    }

    println!("Part 2: {}", result);
}