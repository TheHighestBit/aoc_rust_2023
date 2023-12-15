#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
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
    println!("Part 1: {}", read_file()[0].split(',')
        .fold(0u32, |acc, s| acc + hash(s)));
}

fn part_two() {
    let input = read_file()[0].to_string();
    let steps = input.split(',').collect::<Vec<&str>>();
    let mut boxes: HashMap<u32, VecDeque<(&str, u32)>> = HashMap::new();

    for step in steps {
        if step.contains('-') {
            let label = &step[0..step.len() - 1];
            let hash = hash(label);

            if let Some(deque) = boxes.get_mut(&hash) {
                if let Some(pos) = deque.iter().position(|&x| x.0 == label) {
                    deque.remove(pos);
                }
            }
        } else {
            let splitted = step.split('=').collect::<Vec<&str>>();
            let label = splitted[0];
            let focal_length = splitted[1].parse::<u32>().unwrap();
            let hash = hash(label);

            if let Some(deque) = boxes.get_mut(&hash) {
                if let Some(pos) = deque.iter().position(|&x| x.0 == label) {
                    deque[pos].1 = focal_length;
                } else {
                    deque.push_back((label, focal_length));
                }
            } else {
                let mut deque = VecDeque::new();
                deque.push_back((label, focal_length));
                boxes.insert(hash, deque);
            }
        }
    }

    let mut result: u32 = 0;
    for (i, lens_box) in boxes.iter() {
        for (j, lens) in lens_box.iter().enumerate() {
            result += (i + 1) * (j as u32 + 1) * lens.1;
        }
    }

    println!("Part 2: {}", result);
}

fn hash(s: &str) -> u32 {
    let mut result: u32 = 0;
    for ch in s.chars() {
        result = ((result + ch as u32) * 17) % 256;
    }

    result
}