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
    let mut seed_nr = number_re.captures_iter(&input[0])
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for seed in seed_nr.iter_mut() {
        let mut found = false;
        let mut i = 3;

        while i < input.len() {
            if !input[i].is_empty() && !found {
                // range[0] is dest, range[1] is source, range[2] is length
                let range = number_re.captures_iter(&input[i]).map(|cap| cap[1].parse::<u64>().unwrap()).collect::<Vec<u64>>();

                if *seed >= range[1] && *seed < range[1] + range[2] {
                    *seed = range[0] + (*seed - range[1]);
                    found = true;
                }
            } else if (input[i].is_empty()) {
                i += 1;
                found = false;
            }

            i += 1;
        }
    }

    println!("Part 1: {:?}", seed_nr.iter().min().unwrap());
}

fn part_two() {
    let mut input = read_file();
    input.push(String::new());
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut seed_nr = number_re.captures_iter(&input[0])
        .map(|cap| cap[1].parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Put each translation table into a vector
    let mut translation_tables = Vec::new();

    let mut i = 3;
    while i < input.len() {
        let mut translation_table = Vec::new();

        while !input[i].is_empty() {
            let range = number_re.captures_iter(&input[i]).map(|cap| cap[1].parse::<u64>().unwrap()).collect::<Vec<u64>>();
            translation_table.push(range);
            i += 1;
        }

        translation_tables.push(translation_table);
        i += 2;
    }

    let mut current_ranges = HashSet::new();

    for chunk in seed_nr.chunks(2) {
        current_ranges.insert((chunk[0], chunk[0] + chunk[1] - 1));
    }

    // Loop over each translation table
    for table in translation_tables.iter() {
        let mut table_splits = HashSet::new();

        // Loop over every range and apply it to the table
        for range in current_ranges.iter() {
            let mut range_splits = HashSet::new();
            range_splits.insert(*range);

            // Loop over every line in the table
            for line in table.iter() {
                let mut line_splits = HashSet::new();

                // For every line, check all of the resulting ranges from the previous line
                for range_split in range_splits.iter() {
                    // If there is an overlap, do the split
                    if range_split.1 > line[1] && line[1] + line[2] >= range_split.1 || line[1] + line[2] < range_split.1 && range_split.0 < line[1] + line[2] {
                        // Range before the overlap
                        if range_split.0 < line[1] {
                            line_splits.insert((range_split.0, line[1] - 1));
                        }

                        // Find the overlap and apply the translation
                        let overlap = (range_split.0.max(line[1]), range_split.1.min(line[1] + line[2] - 1));
                        //println!("{:?} -> {:?} -> {:?} -> {:?}", range_split, line, overlap, (line[0] + (overlap.0 - line[1]), line[0] + (overlap.1 - line[1])));
                        table_splits.insert((line[0] + (overlap.0 - line[1]), line[0] + (overlap.1 - line[1])));

                        //println!("{:?} -> {:?} -> {:?}", line_splits.iter().last(), range, line);

                        //println!("{:?} -> {:?} -> {:?} -> Overlap: {:?}", range, line, overlap, range_splits.iter().last());

                        // Range after the overlap
                        if range_split.1 > line[1] + line[2] - 1 {
                            line_splits.insert((line[1] + line[2] - 1, range_split.1));
                        }
                    } else {
                        line_splits.insert(*range_split);
                    }

                    //println!("After line: {:?}", line_splits)
                }

                range_splits = line_splits.clone();
                line_splits.clear();
            }

            table_splits.extend(range_splits.clone());
            range_splits.clear();
        }

        current_ranges = table_splits.clone();
        table_splits.clear();

        //println!("After table: {:?}", current_ranges);
    }

    println!("{:?}", current_ranges.iter().map(|x| x.0).min().unwrap());
}