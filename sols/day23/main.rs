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

type Route = ((usize, usize), Vec<(usize, usize)>);

fn part_one() {
    let input = read_file();
    let grid = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Since the grid is quite constrained, we can just brute force it.
    let start_pos: Route = ((1, 0), Vec::new());
    let mut routes: Vec<Route> = vec![start_pos];
    let mut final_routes: Vec<Route> = Vec::new();

    while let Some(current_route) = routes.pop() {
        let mut current_pos = current_route.0;
        let mut path = current_route.1.clone();
        let mut new_steps = Vec::new();

        if current_pos.0 == grid[0].len() - 2 && current_pos.1 == grid.len() - 1 {
            final_routes.push(current_route);
            continue;
        }

        match grid[current_pos.1][current_pos.0] {
            '>' => {
                current_pos.0 += 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            },
            '<' => {
                current_pos.0 -= 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            },
            '^' => {
                current_pos.1 -= 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            },
            'v' => {
                current_pos.1 += 1;
                if !path.contains(&current_pos) {
                    new_steps.push(current_pos);
                }
            },
            _ => {
                if current_pos.0 > 0 && grid[current_pos.1][current_pos.0 - 1] != '#' && !path.contains(&(current_pos.0 - 1, current_pos.1)) {
                    new_steps.push((current_pos.0 - 1, current_pos.1));
                }
                if current_pos.0 < grid[0].len() - 1 && grid[current_pos.1][current_pos.0 + 1] != '#' && !path.contains(&(current_pos.0 + 1, current_pos.1)) {
                    new_steps.push((current_pos.0 + 1, current_pos.1));
                }
                if current_pos.1 > 0 && grid[current_pos.1 - 1][current_pos.0] != '#' && !path.contains(&(current_pos.0, current_pos.1 - 1)) {
                    new_steps.push((current_pos.0, current_pos.1 - 1));
                }
                if current_pos.1 < grid.len() - 1 && grid[current_pos.1 + 1][current_pos.0] != '#' && !path.contains(&(current_pos.0, current_pos.1 + 1)) {
                    new_steps.push((current_pos.0, current_pos.1 + 1));
                }
            },
        }

        for step in new_steps {
            let mut new_path = path.clone();
            new_path.push(step);
            routes.push((step, new_path));
        }
    }

    println!("Part 1: {}", final_routes.iter().map(|r| r.1.len()).max().unwrap());
}

fn part_two() {
    let input = read_file();
    let mut grid = input.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    // Replace the slopes with .
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '>' || grid[y][x] == '<' || grid[y][x] == '^' || grid[y][x] == 'v' {
                grid[y][x] = '.';
            }
        }
    }

    // Since the grid is quite constrained, we can just brute force it.
    let start_pos: Route = ((1, 0), Vec::new());
    let mut routes: Vec<Route> = vec![start_pos];
    let mut final_routes: Vec<Route> = Vec::new();

    while let Some(current_route) = routes.pop() {
        let mut current_pos = current_route.0;
        let mut path = current_route.1.clone();
        let mut new_steps = Vec::new();

        if current_pos.0 == grid[0].len() - 2 && current_pos.1 == grid.len() - 1 {
            final_routes.push(current_route);
            continue;
        }

        if current_pos.0 > 0 && grid[current_pos.1][current_pos.0 - 1] != '#' && !path.contains(&(current_pos.0 - 1, current_pos.1)) {
            new_steps.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < grid[0].len() - 1 && grid[current_pos.1][current_pos.0 + 1] != '#' && !path.contains(&(current_pos.0 + 1, current_pos.1)) {
            new_steps.push((current_pos.0 + 1, current_pos.1));
        }
        if current_pos.1 > 0 && grid[current_pos.1 - 1][current_pos.0] != '#' && !path.contains(&(current_pos.0, current_pos.1 - 1)) {
            new_steps.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < grid.len() - 1 && grid[current_pos.1 + 1][current_pos.0] != '#' && !path.contains(&(current_pos.0, current_pos.1 + 1)) {
            new_steps.push((current_pos.0, current_pos.1 + 1));
        }

        for step in new_steps {
            let mut new_path = path.clone();
            new_path.push(step);
            routes.push((step, new_path));
        }
    }

    println!("Part 2: {}", final_routes.iter().map(|r| r.1.len()).max().unwrap());
}