#![allow(unused)]
#![allow(arithmetic_overflow)]

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

type Node = (i32, i32, u32, char, u32); // (x, y, distance, direction, steps)

fn part_one() {
    let input = read_file();
    let grid: Vec<Vec<u32>> = input.iter().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    let mut unvisited: HashSet<Node> = HashSet::new();

    // Create and add all nodes
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for step_count in 1..=3 {
                if y == 0 && x != 0 && x != grid[y].len() - 1 { // Cant enter top row from the top
                    unvisited.insert((x as i32, 0, u32::MAX, '<', step_count)); // Enter from the right
                    unvisited.insert((x as i32, 0, u32::MAX, '>', step_count)); // Enter from the left
                    unvisited.insert((x as i32, 0, u32::MAX, '^', step_count)); // Enter from the bottom
                } else if y == grid.len() - 1 && x != 0 && x != grid[y].len() - 1 {
                    unvisited.insert((x as i32, y as i32, u32::MAX, '<', step_count)); // Enter from the right
                    unvisited.insert((x as i32, y as i32, u32::MAX, '>', step_count)); // Enter from the left
                    unvisited.insert((x as i32, y as i32, u32::MAX, 'v', step_count)); // Enter from the top
                } else if x == 0 && y != 0 && y != grid.len() - 1 {
                    unvisited.insert((0, y as i32, u32::MAX, '^', step_count)); // Enter from the bottom
                    unvisited.insert((0, y as i32, u32::MAX, 'v', step_count)); // Enter from the top
                    unvisited.insert((0, y as i32, u32::MAX, '<', step_count)); // Enter from the right
                } else if x == grid[y].len() - 1 && y != 0 && y != grid.len() - 1 {
                    unvisited.insert((x as i32, y as i32, u32::MAX, '^', step_count)); // Enter from the bottom
                    unvisited.insert((x as i32, y as i32, u32::MAX, 'v', step_count)); // Enter from the top
                    unvisited.insert((x as i32, y as i32, u32::MAX, '>', step_count)); // Enter from the left
                } else if x != 0 && x != grid[y].len() - 1 && y != 0 && y != grid.len() - 1 {
                    unvisited.insert((x as i32, y as i32, u32::MAX, '^', step_count)); // Enter from the bottom
                    unvisited.insert((x as i32, y as i32, u32::MAX, 'v', step_count)); // Enter from the top
                    unvisited.insert((x as i32, y as i32, u32::MAX, '<', step_count)); // Enter from the right
                    unvisited.insert((x as i32, y as i32, u32::MAX, '>', step_count)); // Enter from the left
                }
            }
        }
    }

    //Add the corners manually
    for step_count in 1..=3 {
        // Top right
        unvisited.insert(((grid[0].len() - 1) as i32, 0, u32::MAX, '>', step_count));
        unvisited.insert(((grid[0].len() - 1) as i32, 0, u32::MAX, '^', step_count));

        // Bottom right
        unvisited.insert(((grid[0].len() - 1) as i32, (grid.len() - 1) as i32, u32::MAX, '>', step_count));
        unvisited.insert(((grid[0].len() - 1) as i32, (grid.len() - 1) as i32, u32::MAX, 'v', step_count));

        // Bottom left
        unvisited.insert((0, (grid.len() - 1) as i32, u32::MAX, '<', step_count));
        unvisited.insert((0, (grid.len() - 1) as i32, u32::MAX, 'v', step_count));
    }

    let mut current_node: Node = (0, 0, 0, ' ', 1);

    loop {
        match current_node.4 {
            1 | 2 => {
                if current_node.3 != 'v' {
                    let next_steps = match current_node.3 {
                        '^' => current_node.4 + 1,
                        _ => 1
                    };
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0, current_node.1 - 1, '^', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize - 1][current_node.0 as usize] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize - 1][current_node.0 as usize];
                            unvisited.insert(next_node);
                        }
                    }
                }

                if current_node.3 != '^' {
                    let next_steps = match current_node.3 {
                        'v' => current_node.4 + 1,
                        _ => 1
                    };
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0, current_node.1 + 1, 'v', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize + 1][current_node.0 as usize] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize + 1][current_node.0 as usize];
                            unvisited.insert(next_node);
                        }
                    }
                }

                if current_node.3 != '>' {
                    let next_steps = match current_node.3 {
                        '<' => current_node.4 + 1,
                        _ => 1
                    };
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0 - 1, current_node.1, '<', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize][current_node.0 as usize - 1] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize][current_node.0 as usize - 1];
                            unvisited.insert(next_node);
                        }

                        unvisited.insert(next_node);
                    }
                }

                if current_node.3 != '<' {
                    let next_steps = match current_node.3 {
                        '>' => current_node.4 + 1,
                        _ => 1
                    };
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0 + 1, current_node.1, '>', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize][current_node.0 as usize + 1] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize][current_node.0 as usize + 1];
                            unvisited.insert(next_node);
                        }

                        unvisited.insert(next_node);
                    }
                }
            },
            3 => {
                if current_node.3 != 'v' && current_node.3 != '^' {
                    let next_steps = 1;
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0, current_node.1 - 1, '^', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize - 1][current_node.0 as usize] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize - 1][current_node.0 as usize];
                            unvisited.insert(next_node);
                        }
                    }
                }

                if current_node.3 != '^' && current_node.3 != 'v' {
                    let next_steps = 1;
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0, current_node.1 + 1, 'v', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize + 1][current_node.0 as usize] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize + 1][current_node.0 as usize];
                            unvisited.insert(next_node);
                        }
                    }
                }

                if current_node.3 != '>' && current_node.3 != '<' {
                    let next_steps = 1;
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0 - 1, current_node.1, '<', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize][current_node.0 as usize - 1] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize][current_node.0 as usize - 1];
                            unvisited.insert(next_node);
                        }
                    }
                }

                if current_node.3 != '<' && current_node.3 != '>' {
                    let next_steps = 1;
                    if let Some(mut next_node) = find_node(&unvisited, current_node.0 + 1, current_node.1, '>', next_steps) {
                        if next_node.2 > current_node.2 + grid[current_node.1 as usize][current_node.0 as usize + 1] {
                            unvisited.remove(&next_node);
                            next_node.2 = current_node.2 + grid[current_node.1 as usize][current_node.0 as usize + 1];
                            unvisited.insert(next_node);
                        }
                    }
                }
            },
            _ => panic!("Something went wrong!")
        }

        current_node = *unvisited.iter().min_by_key(|n| n.2).unwrap();
        unvisited.remove(&current_node);

        if current_node.0 == grid[0].len() as i32 - 1 && current_node.1 == grid.len() as i32 - 1 {
            println!("Part 1: {}", current_node.2);
            break;
        }
    }
}

fn find_node(nodes: &HashSet<Node>, x: i32, y: i32, direction: char, steps: u32) -> Option<Node> {
    nodes.iter().find(|&&node| node.0 == x && node.1 == y && node.3 == direction && node.4 == steps).copied()
}


fn part_two() {
    // Simply have no more time today
}