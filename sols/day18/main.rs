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

fn part_one() {
    let input = read_file();
    let mut vertices = Vec::new();
    let mut current_pos = (0, 0);
    let mut result = 0;

    for vertice in input.iter() {
        let splitted = vertice.split(' ').collect::<Vec<&str>>();

        let vector = match splitted[0].parse::<char>().unwrap() {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("Invalid direction")
        };

        let distance = splitted[1].parse::<u32>().unwrap();

        for _ in 0..distance {
            current_pos.0 += vector.0;
            current_pos.1 += vector.1;
            vertices.push(current_pos);
        }
    }

    let mut x_range = vertices.iter().min_by_key(|v| v.0).unwrap().0..=vertices.iter().max_by_key(|v| v.0).unwrap().0;
    let mut y_range = vertices.iter().min_by_key(|v| v.1).unwrap().1..=vertices.iter().max_by_key(|v| v.1).unwrap().1;

    for x in x_range {
        for y in y_range.clone() {
            if vertices.contains(&(x, y)) || is_inside(&vertices, (x, y)) {
                result += 1;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part_two() {
    let input = read_file();
    let mut vertices = Vec::new();
    let mut current_pos: (i64, i64) = (0, 0);
    let mut result: i64 = 0;
    let mut boundary_points: u64 = 0;

    for vertice in input.iter() {
        let splitted = vertice.split(' ').collect::<Vec<&str>>();
        let instruction = hex_to_instruction(splitted[2]);

        let vector: (i64, i64) = match instruction.0 {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, -1),
            'D' => (0, 1),
            _ => panic!("Invalid direction")
        };

        let distance: i64 = instruction.1;
        let vertice_range = ((current_pos.0, current_pos.1), (current_pos.0 + vector.0 * distance, current_pos.1 + vector.1 * distance));
        vertices.push(vertice_range);

        current_pos.0 += vector.0 * distance;
        current_pos.1 += vector.1 * distance;
        boundary_points += distance as u64;
    }

    println!("Part 2: {}", total_area(&vertices) + (boundary_points as f64 / 2.0) + 1.0); //No clue why it's +1 instead of -1 as in Pick's
}

fn is_inside(vertices: &Vec<(i32, i32)>, point: (i32, i32)) -> bool {
    let mut intersections = 0;
    let mut prev_vertex = *vertices.iter().last().unwrap(); // assuming non-empty HashSet

    for &current_vertex in vertices {
        let (low_vertex, high_vertex) = if prev_vertex.1 < current_vertex.1 {
            (prev_vertex, current_vertex)
        } else {
            (current_vertex, prev_vertex)
        };

        // Check if the horizontal ray from the point intersects with the edge (low_vertex, high_vertex)
        if point.1 > low_vertex.1 && point.1 <= high_vertex.1 && point.0 <= high_vertex.0.max(low_vertex.0) {
            let edge_orientation = (high_vertex.0 - low_vertex.0) * (point.1 - low_vertex.1)
                - (point.0 - low_vertex.0) * (high_vertex.1 - low_vertex.1);

            if edge_orientation == 0 {
                // The point is on the edge, can be considered inside or on boundary
                return true;
            }
            if edge_orientation > 0 {
                intersections += 1;
            }
        }
        prev_vertex = current_vertex;
    }

    intersections % 2 != 0
}

fn hex_to_instruction(hex: &str) -> (char, i64){
    let steps = i64::from_str_radix(&hex[2..hex.len() - 2], 16).unwrap();
    let direction = match hex.chars().nth(hex.len() - 2).unwrap() {
        '0' => 'R',
        '1' => 'D',
        '2' => 'L',
        '3' => 'U',
        _ => panic!("Invalid direction")
    };

    (direction, steps)
}

fn segment_area_contribution(start: (i64, i64), end: (i64, i64)) -> f64 {
    let (x1, y1) = start;
    let (x2, y2) = end;
    ((x2 - x1) as f64) * ((y1 + y2) as f64) / 2.0
}

fn total_area(segments: &Vec<((i64, i64), (i64, i64))>) -> f64 {
    segments.iter()
        .map(|&(start, end)| segment_area_contribution(start, end))
        .sum::<f64>()
        .abs()
}