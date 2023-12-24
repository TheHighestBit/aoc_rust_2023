#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::DirEntryExt;
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

type HailStone = ((i128, i128, i128), (i128, i128, i128));
type Line = (i128, i128);

fn part_one() {
    let input = read_file();
    let mut hailstones: Vec<HailStone> = Vec::new();
    let mut counter = 0;
    let boundary = 200000000000000..=400000000000000;

    for line in input.iter() {
        let mut split = line.split(" @ ");
        let mut pos = split.next().unwrap()
            .split(", ")
            .map(|x| x.trim().parse::<i128>())
            .collect::<Result<Vec<i128>, _>>()
            .unwrap();
        let mut vel = split.next().unwrap()
            .split(", ")
            .map(|x| x.trim().parse::<i128>())
            .collect::<Result<Vec<i128>, _>>()
            .unwrap();
    
        let coordinates = (pos[0], pos[1], pos[2]);
        let velocity = (vel[0], vel[1], vel[2]);
    
        hailstones.push((coordinates, velocity));
    }

    for (i, hailstone1) in hailstones.iter().enumerate() {
        for (j, hailstone2) in hailstones.iter().enumerate().skip(i + 1) {
            let (coordinates1, velocity1) = hailstone1;
            let (coordinates2, velocity2) = hailstone2;
            let (x1, y1, z1) = coordinates1;
            let (x3, y3, z3) = coordinates2;
    
            let denominator = (x1 - (x1 + velocity1.0)) * (y3 - (y3 + velocity2.1)) - (y1 - (y1 + velocity1.1)) * (x3 - (x3 + velocity2.0));
    
            if denominator != 0 { // Not parallel
                let px = ((x1 * (y1 + velocity1.1) - y1 * (x1 + velocity1.0)) * (x3 - (x3 + velocity2.0)) - (x1 - (x1 + velocity1.0)) * (x3 * (y3 + velocity2.1) - y3 * (x3 + velocity2.0))) / denominator;
                let py = ((x1 * (y1 + velocity1.1) - y1 * (x1 + velocity1.0)) * (y3 - (y3 + velocity2.1)) - (y1 - (y1 + velocity1.1)) * (x3 * (y3 + velocity2.1) - y3 * (x3 + velocity2.0))) / denominator;
    
                if boundary.contains(&px) && boundary.contains(&py) {
                    let t1 = (px - x1) / velocity1.0;
                    let t2 = (px - x3) / velocity2.0;

                    if t1 > 0 && t2 > 0 {
                        counter += 1;
                    }
                }
            }
        }
    }
    

    println!("{}", counter);
}

fn part_two() {
    // This should be done with z3
}