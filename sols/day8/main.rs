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
    let input = read_file();
    let instructions = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let node_re = Regex::new(r"[A-Z]+").unwrap();

    for i in 2..input.len() {
        let cap = node_re.find_iter(input.get(i).unwrap()).map(|m| m.as_str()).collect::<Vec<&str>>();
        
        nodes.insert(cap.first().unwrap(), (cap.get(1).unwrap(), cap.get(2).unwrap()));
    }

    let mut steps = 0;
    let mut instruction_index = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }

        match instructions[instruction_index] {
            'R' => {
                current_node = nodes.get(current_node).unwrap().1;
                instruction_index += 1;
            },
            'L' => {
                current_node = nodes.get(current_node).unwrap().0;
                instruction_index += 1;
            },
            _ => {
                break;
            }
        }

        steps += 1;
    }

    println!("Part 1: {}", steps);
}

fn part_two() {
    let input = read_file();
    let instructions = input.get(0).unwrap().chars().collect::<Vec<char>>();
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    let node_re = Regex::new(r"[A-Z]+").unwrap();

    for i in 2..input.len() {
        let cap = node_re.find_iter(input.get(i).unwrap()).map(|m| m.as_str()).collect::<Vec<&str>>();
        
        nodes.insert(cap.first().unwrap(), (cap.get(1).unwrap(), cap.get(2).unwrap()));
    }

    let mut steps = 0;
    let mut instruction_index = 0;
    let mut current_nodes = nodes.iter().filter(|(k, v)| k.ends_with('A')).map(|(k, v)| k).collect::<Vec<&&str>>();
    let mut routes = HashMap::new();

    for node in current_nodes {
        let mut route = Vec::new();
        let mut current_node = node;
        
        while !current_node.ends_with('Z') {
            if instruction_index == instructions.len() {
                instruction_index = 0;
            }

            route.push(instructions[instruction_index]);

            match instructions[instruction_index] {
                'R' => {
                    current_node = &nodes.get(current_node).unwrap().1;
                },
                'L' => {
                    current_node = &nodes.get(current_node).unwrap().0;
                },
                _ => {
                    break;
                }
            }

            instruction_index += 1;
        }

        routes.insert(node, route);

    }

    let values = routes.values().collect::<Vec<&Vec<char>>>();
    let mut current_lcm = values[0].len() as u64;

    for node_route in routes.values() {
        current_lcm = lcm(current_lcm, node_route.len() as u64);
    }

    println!("Part 2: {}", current_lcm);
}

fn gcd(a: u64, b: u64) -> u64 {
    if (b==0)
    {
        return a;
    }
    
    gcd(b,a%b)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a/gcd(a,b))*b
}