#![allow(unused)]

use std::collections::HashMap;
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
    let id_re = Regex::new(r"Game (\d+):").unwrap();
    let cube_re = Regex::new(r"(\d+) (green|blue|red)").unwrap();

    let mut colors: HashMap<&str, i32> = HashMap::new();
    colors.insert("red", 12);
    colors.insert("green", 13);
    colors.insert("blue", 14);

    let mut sum = 0;
    for line in read_file() {
        let mut is_possible = true;
        let id = id_re.captures(&line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let (_, cube_sets) = line.split_at(line.find(':').unwrap() + 2);

        for cube_set in cube_sets.split("; ") {
            for cube in cube_set.split(", ") {
                let count = cube_re.captures(cube).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                let color = cube_re.captures(cube).unwrap().get(2).unwrap().as_str();

                if colors.get(color).unwrap() < &count {
                    is_possible = false;
                }
            }
        }

        if is_possible {
            sum += id;
        }

        is_possible = true;
    }

    println!("Part 1: {}", sum);
}

fn part_two() {
    let id_re = Regex::new(r"Game (\d+):").unwrap();
    let cube_re = Regex::new(r"(\d+) (green|blue|red)").unwrap();

    let mut colors: HashMap<String, i32> = HashMap::new();
    colors.insert(String::from("red"), 0);
    colors.insert(String::from("green"), 0);
    colors.insert(String::from("blue"), 0);

    let mut sum = 0;
    for line in read_file() {
        let id = id_re.captures(&line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let (_, cube_sets) = line.split_at(line.find(':').unwrap() + 2);

        for cube_set in cube_sets.split("; ") {
            for cube in cube_set.split(", ") {
                let count = cube_re.captures(cube).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
                let color = cube_re.captures(cube).unwrap().get(2).unwrap().as_str();

                if colors.get(color).unwrap() < &count {
                    colors.insert(color.to_string(), count);
                }
            }
        }
        sum += colors.values().product::<i32>();

        for value in colors.values_mut() {
            *value = 0;
        }
    }

    println!("Part 2: {}", sum);
}