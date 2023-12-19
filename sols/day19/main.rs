#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

type Rule = (char, char, i32, String); // Category, operation (either < or >), amount and resulting destination
type Part = (i32, i32, i32, i32);
fn part_one() {
    let input = read_file();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: HashSet<Part> = HashSet::new();
    let mut is_parts = false;
    let mut result = 0;

    for line in input {
        if line.is_empty() {
            is_parts = true;
            continue;
        }

        if !is_parts {
            let (name, rules) = parse_workflow(&line);
            workflows.insert(name, rules);
        } else {
            parts.insert(parse_part(&line));
        }
    }

    let mut current_workflow = "in";

    for part in parts {
        if check_part(&part, &workflows) {
            result += part.0 + part.1 + part.2 + part.3;
        }
    }

    println!("Part 1: {}", result);
}

fn part_two() {
    let input = read_file();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: HashSet<Part> = HashSet::new();
    let mut result = 0;

    for line in input {
        if line.is_empty() {
            break;
        }

        let (name, rules) = parse_workflow(&line);
        workflows.insert(name, rules);
    }

    // Find all breakpoints for part values
    let mut breakpoints: HashMap<char, Vec<i32>> = HashMap::new();

    for workflow in workflows.values() {
        for rule in workflow {
            if rule.0 != ' ' {
                let mut breakpoints_for_category = breakpoints.entry(rule.0).or_default();
                breakpoints_for_category.push(rule.2);
            }
        }
    }

    for breakpoints_for_category in breakpoints.values_mut() {
        breakpoints_for_category.push(1);
        breakpoints_for_category.push(4000);
        breakpoints_for_category.sort();
        breakpoints_for_category.dedup();
    }

    breakpoints.iter_mut().for_each(|(_, v)| v.sort());

    let mut total_count: u64 = 0;
    for x_range in breakpoints[&'x'].windows(2) {
        for m_range in breakpoints[&'m'].windows(2) {
            for a_range in breakpoints[&'a'].windows(2) {
                for s_range in breakpoints[&'s'].windows(2) {
                    let representative_part: Part = (x_range[0] + 1, m_range[0] + 1, a_range[0] + 1, s_range[0] + 1);

                    if check_part(&representative_part, &workflows) {
                        let count: u64 = ((x_range[1] - x_range[0]) as u64)
                            * ((m_range[1] - m_range[0]) as u64)
                            * ((a_range[1] - a_range[0]) as u64)
                            * ((s_range[1] - s_range[0]) as u64);
                        total_count += count;
                    }
                }
            }
        }
    }


    println!("Part 2: {}", total_count);
}

fn parse_workflow(line: &str) -> (String, Vec<Rule>) {
    let splitted = line.split('{').collect::<Vec<&str>>();
    let name = splitted[0].to_string();
    let rules_splitted = splitted[1].split(',').collect::<Vec<&str>>();
    let mut rules: Vec<Rule> = Vec::new();

    for (i, rule) in rules_splitted.iter().enumerate() {
        if i == rules_splitted.len() - 1 {
            let mut rule_string = rule.to_string();
            rule_string.pop();
            rules.push((' ', ' ', 0, rule_string));
            break;
        }

        let mut rule_splitted = rule.split(':');
        let condition = rule_splitted.next().unwrap();
        let destination = rule_splitted.next().unwrap();

        let mut condition_splitted = if condition.contains('<') {
            condition.split('<')
        } else {
            condition.split('>')
        };

        let category = condition_splitted.next().unwrap().chars().next().unwrap();
        let amount = condition_splitted.next().unwrap().parse::<i32>().unwrap();

        rules.push((category, if condition.contains('<') { '<' } else { '>' }, amount, destination.to_string()));
    }

    (name, rules)
}

fn parse_part(line: &str) -> Part {
    let trimmed_line = &line[1..line.len()-1];
    let parts = trimmed_line.split(',').collect::<Vec<&str>>();

    let x = parts[0].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let m = parts[1].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let a = parts[2].split('=').nth(1).unwrap().parse::<i32>().unwrap();
    let s = parts[3].split('=').nth(1).unwrap().parse::<i32>().unwrap();

    (x, m, a, s)
}

fn check_part(part: &Part, workflows: &HashMap<String, Vec<Rule>>) -> bool {
    let mut current_workflow = "in";

    'outer: loop {
        if current_workflow == "A" {
            return true;
        } else if current_workflow == "R" {
            return false;
        }

        let rules = workflows.get(current_workflow).unwrap();

        for (i, rule) in rules.iter().enumerate() {
            if i == rules.len() - 1 {
                match rule.3.as_str() {
                    "A" => {
                        return true;
                    },
                    "R" => {
                        return false;
                    },
                    x => {
                        current_workflow = x;
                    }
                }
            } else {
                let part_value = match rule.0 {
                    'x' => part.0,
                    'm' => part.1,
                    'a' => part.2,
                    's' => part.3,
                    _ => continue,
                };

                match rule.1 {
                    '<' if part_value < rule.2 => current_workflow = &rule.3,
                    '>' if part_value > rule.2 => current_workflow = &rule.3,
                    _ => continue,
                }
                break;
            }
        }
    }
}