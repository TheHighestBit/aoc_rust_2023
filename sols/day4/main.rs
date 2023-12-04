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
    let number_re = Regex::new(r" (\d+)").unwrap();
    let mut sum = 0;

    for line in read_file() {
        // The first number is the card number, then 10 winners and 25 our numbers
        let numbers: Vec<i32> = number_re.captures_iter(&line).map(|cap| cap[1].parse::<i32>().unwrap()).collect();

        let winning_numbers: HashSet<i32> = HashSet::from_iter(numbers[1..11].iter().cloned());
        let our_numbers: HashSet<i32> = HashSet::from_iter(numbers[11..36].iter().cloned());

        let matches = winning_numbers.intersection(&our_numbers).count() as u32;

        if matches > 0 {
            sum += 2_i32.pow(matches - 1);
        }
    }

    println!("Part 1: {}", sum);
}

fn part_two() {
    let number_re = Regex::new(r" (\d+)").unwrap();
    let mut sum = 0;
    let cards = read_file();
    
    // Keep track of how many copies of each card we have
    let mut card_copies: HashMap<i32, i32> = HashMap::new();
    card_copies.insert(0, 0);

    for i in 0..cards.len() {
        let numbers: Vec<i32> = number_re.captures_iter(&cards[i]).map(|cap| cap[1].parse::<i32>().unwrap()).collect();

        let winning_numbers: HashSet<i32> = HashSet::from_iter(numbers[1..11].iter().cloned());
        let our_numbers: HashSet<i32> = HashSet::from_iter(numbers[11..36].iter().cloned());

        let matches = winning_numbers.intersection(&our_numbers).count();

        if matches > 0 {
            let copy_count = card_copies.get(&(i as i32)).copied().unwrap_or_default();
            for j in i + 1..matches + i + 1 {
                *card_copies.entry(j as i32).or_insert(0) += copy_count + 1;
            }
        }
    }

    println!("Part 2: {}", card_copies.values().sum::<i32>() + cards.len() as i32);
}