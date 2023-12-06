#![allow(unused)]

use std::cmp::{max, min};
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
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut answer = 1;

    let time = number_re.captures_iter(&input[0]).map(|x| x[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let distance = number_re.captures_iter(&input[1]).map(|x| x[0].parse::<i32>().unwrap()).collect::<Vec<i32>>();

    for i in 0..time.len() {
        let mut nr_ways = 0;

        for millisecond in 1..time[i] {
            let time_left = time[i] - millisecond;
            let distance_traveled = millisecond * time_left;

            if distance_traveled > distance[i] {
                nr_ways += 1;
            }
        }

        answer *= nr_ways;
    }

    println!("Part 1: {}", answer);
}

fn part_two() {
    let input = read_file();
    let number_re = Regex::new(r"(\d+)").unwrap();

    let time = number_re.captures_iter(&input[0].replace(' ', "")).map(|x| x[0].parse::<u64>().unwrap()).collect::<Vec<u64>>()[0];
    let distance = number_re.captures_iter(&input[1].replace(' ', "")).map(|x| x[0].parse::<u64>().unwrap()).collect::<Vec<u64>>()[0];

    let mut nr_ways = 0;

    for millisecond in 1..time {
        let time_left = time - millisecond;
        let distance_traveled = millisecond * time_left;

        if distance_traveled > distance {
            nr_ways += 1;
        }
    }

    println!("Part 2: {}", nr_ways);
}