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
    let mut pipe2dir = HashMap::new();
    pipe2dir.insert('|', ('N', 'S'));
    pipe2dir.insert('-', ('E', 'W'));
    pipe2dir.insert('L', ('N', 'E'));
    pipe2dir.insert('7', ('S', 'W'));
    pipe2dir.insert('J', ('N', 'W'));
    pipe2dir.insert('F', ('S', 'E'));

    let input = read_file();
    let mut grid = Vec::new();

    for line in input {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut start = (0, 0);

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'S' {
                start = (x, y);
                break;
            }
        }
    };

    let mut dir = 'S';
    let mut pos = start;

    // Check north of start
    if pos.0 > 0 && ['|', '7', 'F'].contains(&grid[pos.0 - 1][pos.1]) {
        dir = 'N';
        pos.0 -= 1;
    }

    // Check east of start
    else if pos.1 < grid[pos.0].len() - 1 && ['-', 'J', '7'].contains(&grid[pos.0][pos.1 + 1]) {
        dir = 'E';
        pos.1 += 1;
    }

    // Check south of start
    else if pos.0 < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[pos.0 + 1][pos.1]) {
        dir = 'S';
        pos.0 += 1;
    }

    // No need to check for east since start will have 2 pipes connected to it

    let mut route = Vec::new();

    loop {
        route.push(grid[pos.0][pos.1]);

        if grid[pos.0][pos.1] == 'S' {
            break;
        }

        // Translate directions
        dir = match dir {
            'N' => 'S',
            'E' => 'W',
            'S' => 'N',
            'W' => 'E',
            _ => panic!("Invalid direction")
        };

        // Find direction of next pipe
        let next_dir = if pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0 == dir {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().1
        } else {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0
        };

        pos = match next_dir {
            'N' => (pos.0 - 1, pos.1),
            'E' => (pos.0, pos.1 + 1),
            'S' => (pos.0 + 1, pos.1),
            'W' => (pos.0, pos.1 - 1),
            _ => panic!("Invalid direction")
        };

        dir = next_dir;
    }

    println!("Part 1: {}", route.len() / 2);
}

fn part_two() {
    let mut pipe2dir = HashMap::new();
    pipe2dir.insert('|', ('N', 'S'));
    pipe2dir.insert('-', ('E', 'W'));
    pipe2dir.insert('L', ('N', 'E'));
    pipe2dir.insert('7', ('S', 'W'));
    pipe2dir.insert('J', ('N', 'W'));
    pipe2dir.insert('F', ('S', 'E'));

    let input = read_file();
    let mut grid = Vec::new();

    for line in input {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let mut start = (0, 0);

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == 'S' {
                start = (x, y);
                break;
            }
        }
    };

    let mut dir = 'S';
    let mut pos = start;

    // Check north of start
    if pos.0 > 0 && ['|', '7', 'F'].contains(&grid[pos.0 - 1][pos.1]) {
        dir = 'N';
        pos.0 -= 1;
    }

    // Check east of start
    else if pos.1 < grid[pos.0].len() - 1 && ['-', 'J', '7'].contains(&grid[pos.0][pos.1 + 1]) {
        dir = 'E';
        pos.1 += 1;
    }

    // Check south of start
    else if pos.0 < grid.len() - 1 && ['|', 'L', 'J'].contains(&grid[pos.0 + 1][pos.1]) {
        dir = 'S';
        pos.0 += 1;
    }

    // No need to check for east since start will have 2 pipes connected to it

    let mut route = Vec::new();

    loop {
        route.push((pos.0 as f64, pos.1 as f64));

        if grid[pos.0][pos.1] == 'S' {
            break;
        }

        // Translate directions
        dir = match dir {
            'N' => 'S',
            'E' => 'W',
            'S' => 'N',
            'W' => 'E',
            _ => panic!("Invalid direction")
        };

        // Find direction of next pipe
        let next_dir = if pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0 == dir {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().1
        } else {
            pipe2dir.get(&grid[pos.0][pos.1]).unwrap().0
        };

        pos = match next_dir {
            'N' => (pos.0 - 1, pos.1),
            'E' => (pos.0, pos.1 + 1),
            'S' => (pos.0 + 1, pos.1),
            'W' => (pos.0, pos.1 - 1),
            _ => panic!("Invalid direction")
        };

        dir = next_dir;
    }

    let mut counter = 0;

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if !route.contains(&(x as f64, y as f64)) && is_point_in_polygon((x as f64, y as f64), &route) {
                counter += 1;
            }
        }
    }

    println!("Part 2: {}", counter);
}

fn is_point_in_polygon(point: (f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
    let mut is_inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];

        let intersect = ((yi > point.1) != (yj > point.1))
            && (point.0 < (xj - xi) * (point.1 - yi) / (yj - yi) + xi);

        if intersect {
            is_inside = !is_inside;
        }

        j = i;
    }

    is_inside
}
