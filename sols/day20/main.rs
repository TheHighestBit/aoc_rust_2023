#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::str::Chars;
use regex::Regex;
use serde::{Deserialize, Serialize};

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

type Conjunction = (HashMap<String, bool>);

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
enum Node {
    FlipFlop(bool, Vec<String>),
    Conjunction(Conjunction, Vec<String>),
    Broadcaster(Vec<String>)
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Node::FlipFlop(flip_flop, ..) => {
                "FlipFlop".hash(state);
                flip_flop.hash(state);
            },
            Node::Conjunction(conjunction, ..) => {
                "Conjunction".hash(state);
                let mut keys: Vec<_> = conjunction.keys().collect();
                keys.sort();
                for key in keys {
                    key.hash(state);
                    conjunction.get(key).unwrap().hash(state);
                }
            },
            Node::Broadcaster(broadcasters) => {
                "Broadcaster".hash(state);
                for broadcaster in broadcasters {
                    broadcaster.hash(state);
                }
            },
        }
    }
}

fn part_one() {
    let input = read_file();
    let mut modules: HashMap<String, Node> = HashMap::new();

    // First pass is to find the Conjunction modules
    for line in input.iter() {
        if line.contains('&') {
            let mut splitted = line[1..].split(" -> ");
            let node_label = splitted.next().unwrap();
            let mut destination_nodes_split = splitted.next().unwrap().split(", ");

            let mut destination_nodes = Vec::new();
            for node in destination_nodes_split {
                destination_nodes.push(node.to_string());
            }

            let mut conjunction: Conjunction = HashMap::new();
            modules.insert(node_label.to_string(), Node::Conjunction(conjunction, destination_nodes));
        }
    }

    // Second pass to find other modules and fill the Conjunction modules
    for line in input.iter() {
        let mut splitted = line[1..].split(" -> ");
        let node_label = if line.contains('%') {
            splitted.next().unwrap().to_string()
        } else {
            splitted.next().unwrap();
            "broadcaster".to_string()
        };

        let destination_nodes_split = splitted.next().unwrap().split(", ");
        let destination_nodes = destination_nodes_split.map(|node| node.to_string()).collect::<Vec<String>>();

        // Check if any of the destination nodes are Conjunction modules
        for node in &destination_nodes {
            if let Some(Node::Conjunction(conjunction, _)) = modules.get_mut(node.as_str()) {
                conjunction.insert(node_label.to_string(), false);
            }
        }

        if line.contains('%') {
            modules.insert(node_label, Node::FlipFlop(false, destination_nodes));
        } else if !line.contains('&') {
            modules.insert(node_label, Node::Broadcaster(destination_nodes));
        }
    }

    let mut states: Vec<(String, (u32, u32))> = Vec::new();
    let mut hash = serialize_map(&modules).unwrap();
    let mut emergency_break = 0;

    while !states.iter().any(|(h, _)| h == &hash) && emergency_break < 1000 {
        emergency_break += 1;
        let mut counter = (1, 0); // A low pulse is sent to the broadcaster
        let mut processing_queue: VecDeque<(String, String, bool)> = VecDeque::new();

        // Send a low pulse to all broadcaster destination nodes
        if let Some(Node::Broadcaster(destination_nodes)) = modules.get("broadcaster") {
            for dest in destination_nodes {
                processing_queue.push_back(("broadcaster".to_string(), dest.to_string(), false));
            }

            counter.0 += destination_nodes.len();
        } else {
            panic!("No broadcaster node found");
        };


        while !processing_queue.is_empty() {
            let (src_node, dest_node, pulse) = processing_queue.pop_front().unwrap();
            if let Some(node) = modules.get_mut(&dest_node) {
                match node {
                    Node::FlipFlop(state, destinations) => {
                        if !pulse {
                            if *state {
                                counter.1 += destinations.len();
                            } else {
                                counter.0 += destinations.len();
                            }
                            *state = !*state;

                            for destination_node in destinations {
                                processing_queue.push_back((dest_node.to_string(), destination_node.to_string(), *state));
                            }
                        }
                    },
                    Node::Conjunction(conjunction, destinations) => {
                        conjunction.entry(src_node.to_string()).and_modify(|e| *e = pulse);

                        let new_pulse = if conjunction.values().all(|&v| v) {
                            counter.0 += destinations.len();
                            false
                        } else {
                            counter.1 += destinations.len();
                            true
                        };

                        for destination_node in destinations {
                            processing_queue.push_back((dest_node.to_string(), destination_node.to_string(), new_pulse));
                        }
                    },
                    _ => {
                        panic!("Unknown node type");
                    }
                }
            }
        }
        states.push((hash, (counter.0 as u32, counter.1 as u32)));
        hash = serialize_map(&modules).unwrap();
    }

    let complete_cycles: u32 = 1000 / states.len() as u32;
    let remaining_cycles = 1000 - complete_cycles * states.len() as u32;
    let mut counter = (0, 0);

    counter.0 += states.iter().fold(0, |acc, (_, b)| acc + b.0 * complete_cycles);
    counter.1 += states.iter().fold(0, |acc, (_, b)| acc + b.1 * complete_cycles);
    counter.0 += states.iter().take(remaining_cycles as usize).fold(0, |acc, (_, b)| acc + b.0);
    counter.1 += states.iter().take(remaining_cycles as usize).fold(0, |acc, (_, b)| acc + b.1);

    println!("Part 1: {}", counter.0 * counter.1);
}

fn part_two() {
    //cba
}

fn serialize_map(map: &HashMap<String, Node>) -> Result<String, serde_json::Error> {
    serde_json::to_string(map)
}