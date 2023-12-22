#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::DirEntryExt;
use regex::Regex;

fn main() {
    // Both are brute force and take ~5 mins to run.
    //part_one();
    part_two();
}

fn read_file() -> Vec<String> {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

type Brick = ((i32, i32, i32), (i32, i32, i32));
type Cube = (i32, i32, i32);

fn part_one() {
    let input = read_file();
    let mut grid: Vec<Brick> = input.iter().map(|f| {
        let mut splitted = f.split('~');
        let mut start = splitted.next().unwrap().split(',').map(|s| s.parse::<i32>().unwrap());
        let mut end = splitted.next().unwrap().split(',').map(|s| s.parse::<i32>().unwrap());
        ((start.next().unwrap(), start.next().unwrap(), start.next().unwrap()), (end.next().unwrap(), end.next().unwrap(), end.next().unwrap()))
    }).collect::<Vec<Brick>>();

    // Move bricks downward till they come to a rest
    let max_z = grid.iter().map(|b| b.1.2).max().unwrap();
    let mut processed = HashSet::new();

    let mut is_change = true;
    let mut change_count = 0;
    while is_change {
        println!("Change count: {}", change_count);
        change_count += 1;
        is_change = false;
        processed.clear();

        for z in 2..=max_z {
            for brick_index in 0..grid.len() {
                let mut brick = grid[brick_index];
                // Check if this brick intersects this z plane
                if !processed.contains(&brick) && brick.0.2 == z {
                    let mut intersects = false;
                    // Check below it along the x and y axes
                    'outer: for brick2 in grid.iter().filter(|el| brick != **el) {
                        if brick2.1.2 == z - 1 { // The brick is directly below it
                            let mut brick_cords = HashSet::new();
                            let mut brick2_cords = HashSet::new();

                            for x in brick.0.0..=brick.1.0 {
                                for y in brick.0.1..=brick.1.1 {
                                    brick_cords.insert((x, y));
                                }
                            }

                            for x in brick2.0.0..=brick2.1.0 {
                                for y in brick2.0.1..=brick2.1.1 {
                                    brick2_cords.insert((x, y));
                                }
                            }

                            if !brick_cords.is_disjoint(&brick2_cords) {
                                intersects = true;
                                break 'outer;
                            }
                        } 
                    }
    
                    if !intersects {
                        // Move the brick down
                        brick.0.2 -= 1;
                        brick.1.2 -= 1;
    
                        grid[brick_index] = brick;
                        is_change = true;
                    }
    
                    processed.insert(brick);
                }
            }
        }
    }

    // Try removing each brick and see any bricks would fall as a result
    let mut counter = 0;

    for current_brick in 0..grid.len() {
        println!("Current brick: {}", current_brick);
        let mut is_stable = true;
        processed.clear();

        // Check stability of each brick with current_brick removed
        for z in 2..=max_z {
            for brick_index in 0..grid.len() {
                if current_brick != brick_index {
                    let mut brick = grid[brick_index];

                    if !processed.contains(&brick) && brick.0.2 == z {
                        let mut intersects = false;

                        // Check for intersections with other bricks
                        for brick2 in grid.iter().enumerate().filter(|(idx, _)| *idx != current_brick && *idx != brick_index) {
                            let (_, brick2) = brick2;

                            if brick2.1.2 == z - 1 {
                                let mut brick_cords = HashSet::new();
                                    let mut brick2_cords = HashSet::new();
        
                                    for x in brick.0.0..=brick.1.0 {
                                        for y in brick.0.1..=brick.1.1 {
                                            brick_cords.insert((x, y));
                                        }
                                    }
        
                                    for x in brick2.0.0..=brick2.1.0 {
                                        for y in brick2.0.1..=brick2.1.1 {
                                            brick2_cords.insert((x, y));
                                        }
                                    }
        
                                    if !brick_cords.is_disjoint(&brick2_cords) {
                                        intersects = true;
                                        break;
                                    }
                            } 
                        }

                        if !intersects {
                            // This brick becomes unstable due to the removal of current_brick
                            is_stable = false;
                            break;
                        }

                        processed.insert(brick);
                    }
                }
            }
            if !is_stable {
                break;
            }
        }

        if is_stable {
            counter += 1; // Increment only if all bricks remain stable
        }
    }

    println!("Part 1: {}", counter);
}

fn part_two() {
    let input = read_file();
    let mut grid: Vec<Brick> = input.iter().map(|f| {
        let mut splitted = f.split('~');
        let mut start = splitted.next().unwrap().split(',').map(|s| s.parse::<i32>().unwrap());
        let mut end = splitted.next().unwrap().split(',').map(|s| s.parse::<i32>().unwrap());
        ((start.next().unwrap(), start.next().unwrap(), start.next().unwrap()), (end.next().unwrap(), end.next().unwrap(), end.next().unwrap()))
    }).collect::<Vec<Brick>>();

    // Move bricks downward till they come to a rest
    let max_z = grid.iter().map(|b| b.1.2).max().unwrap();
    let mut processed = HashSet::new();

    let mut is_change = true;
    let mut change_count = 0;
    while is_change {
        println!("Change count: {}", change_count);
        change_count += 1;
        is_change = false;
        processed.clear();

        for z in 2..=max_z {
            for brick_index in 0..grid.len() {
                let mut brick = grid[brick_index];
                // Check if this brick intersects this z plane
                if !processed.contains(&brick) && brick.0.2 == z {
                    let mut intersects = false;
                    // Check below it along the x and y axes
                    'outer: for brick2 in grid.iter().filter(|el| brick != **el) {
                        if brick2.1.2 == z - 1 { // The brick is directly below it
                            let mut brick_cords = HashSet::new();
                            let mut brick2_cords = HashSet::new();

                            for x in brick.0.0..=brick.1.0 {
                                for y in brick.0.1..=brick.1.1 {
                                    brick_cords.insert((x, y));
                                }
                            }

                            for x in brick2.0.0..=brick2.1.0 {
                                for y in brick2.0.1..=brick2.1.1 {
                                    brick2_cords.insert((x, y));
                                }
                            }

                            if !brick_cords.is_disjoint(&brick2_cords) {
                                intersects = true;
                                break 'outer;
                            }
                        } 
                    }
    
                    if !intersects {
                        // Move the brick down
                        brick.0.2 -= 1;
                        brick.1.2 -= 1;
    
                        grid[brick_index] = brick;
                        is_change = true;
                    }
    
                    processed.insert(brick);
                }
            }
        }
    }

    // Try removing each brick and see any bricks would fall as a result
    let mut counter = 0;
    let org_grid = grid.clone();

    for current_brick in 0..grid.len() {
        println!("Current brick: {}", current_brick);
        processed.clear();

        // Check stability of each brick with current_brick removed
        for z in 2..=max_z {
            for brick_index in 0..grid.len() {
                if current_brick != brick_index {
                    let mut brick = grid[brick_index];

                    if !processed.contains(&brick) && brick.0.2 == z {
                        let mut intersects = false;

                        // Check for intersections with other bricks
                        for brick2 in grid.iter().enumerate().filter(|(idx, _)| *idx != current_brick && *idx != brick_index) {
                            let (i, brick2) = brick2;

                            if brick2.1.2 == z - 1 {
                                let mut brick_cords = HashSet::new();
                                    let mut brick2_cords = HashSet::new();
        
                                    for x in brick.0.0..=brick.1.0 {
                                        for y in brick.0.1..=brick.1.1 {
                                            brick_cords.insert((x, y));
                                        }
                                    }
        
                                    for x in brick2.0.0..=brick2.1.0 {
                                        for y in brick2.0.1..=brick2.1.1 {
                                            brick2_cords.insert((x, y));
                                        }
                                    }
        
                                    if !brick_cords.is_disjoint(&brick2_cords) {
                                        intersects = true;
                                        break;
                                    }
                            } 
                        }

                        if !intersects {
                            // This brick becomes unstable due to the removal of current_brick
                            counter += 1;
                            brick.0.2 -= 1;
                            brick.1.2 -= 1;
                            grid[brick_index] = brick;
                        }

                        processed.insert(brick);
                    }
                }
            }
        }

        grid = org_grid.clone();
    }

    println!("Part 2: {}", counter);    
}