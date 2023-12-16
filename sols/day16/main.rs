#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
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

type Vector = (i32, i32);
type Coordinate = (u32, u32);
type BeamPos = (Coordinate, Vector);
// For each beam we track current position, direction and the route it has taken
type Beam = (Coordinate, Vector);

fn part_one() {
    let mut grid = read_file().iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut energized_grid = grid.clone();
    // Need a way of detecting beams that get stuck bouncing around, for that remember the route of the beam
    let mut explored_beams = HashSet::new();

    let first_beam = match grid[0][0] {
        '\\' => (0, -1),
        _ => (1, 0)
    };

    let mut beams: Vec<Beam> = vec![((0, 0), first_beam)];
    explored_beams.insert(((0, 0), first_beam));
    energized_grid[0][0] = '#';

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = Vec::new();

        for beam in beams.iter_mut() {
            // Extend the beam along the vector, until we hit a mirror or a wall
            match beam.1 {
                (x, 0) => { // Beam is travelling on the x axis
                    let range: Vec<u32> = match x {
                        1 => (beam.0.0 + 1..grid[0].len() as u32).collect(),
                        -1 => (0..beam.0.0).rev().collect(),
                        _ => panic!("Invalid beam vector")
                    };

                    if range.is_empty() { // Beam has reached the end of the grid
                        continue;
                    }

                    for i in range {
                        match grid[beam.0.1 as usize][i as usize] {
                            '/' => {
                                energized_grid[beam.0.1 as usize][i as usize] = '#';

                                let new_vector = match x {
                                    1 => (0, 1),
                                    -1 => (0, -1),
                                    _ => panic!("Invalid beam vector")
                                };

                                let new_pos: BeamPos = ((i, beam.0.1), new_vector);
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = new_vector;
                                    beam.0.0 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            '\\' => {
                                energized_grid[beam.0.1 as usize][i as usize] = '#';

                                let new_vector = match x {
                                    1 => (0, -1),
                                    -1 => (0, 1),
                                    _ => panic!("Invalid beam vector")
                                };

                                let new_pos: BeamPos = ((i, beam.0.1), new_vector);
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = new_vector;
                                    beam.0.0 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            '.' | '-' => { // Energize this cell
                                energized_grid[beam.0.1 as usize][i as usize] = '#';
                            },
                            '|' => { // Split the beam into 2
                                energized_grid[beam.0.1 as usize][i as usize] = '#';

                                let new_pos: BeamPos = ((i, beam.0.1), (0, 1));
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = (0, 1);
                                    beam.0.0 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }

                                let new_pos: BeamPos = ((i, beam.0.1), (0, -1));
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = (0, -1);
                                    beam.0.0 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            _ => { // Rest of the cases we do nothing
                            }
                        }
                    }
                },
                (0, y) => { // Beam is travelling on the y axis
                    let range: Vec<u32> = match y {
                        1 => (0..beam.0.1).rev().collect(),
                        -1 => (beam.0.1 + 1..grid.len() as u32).collect(),
                        _ => panic!("Invalid beam vector")
                    };

                    if range.is_empty() {
                        continue;
                    }

                    for i in range {
                        match grid[i as usize][beam.0.0 as usize] {
                            '/' => {
                                energized_grid[i as usize][beam.0.0 as usize] = '#';

                                let new_vector = match y {
                                    1 => (1, 0),
                                    -1 => (-1, 0),
                                    _ => panic!("Invalid beam vector")
                                };

                                let new_pos: BeamPos = ((beam.0.0, i), new_vector);
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = new_vector;
                                    beam.0.1 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            '\\' => {
                                energized_grid[i as usize][beam.0.0 as usize] = '#';

                                let new_vector = match y {
                                    1 => (-1, 0),
                                    -1 => (1, 0),
                                    _ => panic!("Invalid beam vector")
                                };

                                let new_pos: BeamPos = ((beam.0.0, i), new_vector);
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = new_vector;
                                    beam.0.1 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            '.' | '|' => { // Energize this cell
                                energized_grid[i as usize][beam.0.0 as usize] = '#';
                            },
                            '-' => { // Split the beam into 2
                                energized_grid[i as usize][beam.0.0 as usize] = '#';

                                let new_pos: BeamPos = ((beam.0.0, i), (1, 0));
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = (1, 0);
                                    beam.0.1 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }

                                let new_pos: BeamPos = ((beam.0.0, i), (-1, 0));
                                if !explored_beams.contains(&new_pos) {
                                    beam.1 = (-1, 0);
                                    beam.0.1 = i;
                                    explored_beams.insert((beam.0, beam.1));

                                    new_beams.push(beam.clone());
                                }
                                break;
                            },
                            _ => { // Rest of the cases we do nothing
                            }
                        }
                    }
                },
                _ => panic!("Invalid beam")
            }
        }

        beams = new_beams;
    }

    let result = energized_grid.iter().fold(0, |acc, row| acc + row.iter().filter(|c| **c == '#').count());

    println!("Part 1: {}", result);
}

fn part_two() {
    let mut grid = read_file().iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut energized_grid = grid.clone();
    // Need a way of detecting beams that get stuck bouncing around, for that remember the route of the beam
    let mut beams: Vec<Beam> = Vec::new();
    let mut explored_beams = HashSet::new();
    let mut initial_beams = Vec::new();
    let mut best = 0;

    for i in 0..grid[0].len() {
        match grid[0][i] {
            '/' => {
                initial_beams.push(((i as u32, 0), (-1, 0)));
            },
            '\\' => {
                initial_beams.push(((i as u32, 0), (1, 0)));
            },
            _ => {
                initial_beams.push(((i as u32, 0), (0, -1)));
            }
        }

        match grid.last().unwrap()[i] {
            '/' => {
                initial_beams.push(((i as u32, grid.len() as u32 - 1), (1, 0)));
            },
            '\\' => {
                initial_beams.push(((i as u32, grid.len() as u32 - 1), (-1, 0)));
            },
            _ => {
                initial_beams.push(((i as u32, grid.len() as u32 - 1), (0, 1)));
            }
        }
    }

    for i in 0..grid.len() {
        match grid[i][0] {
            '/' => {
                initial_beams.push(((0, i as u32), (0, 1)));
            },
            '\\' => {
                initial_beams.push(((0, i as u32), (0, -1)));
            },
            _ => {
                initial_beams.push(((0, i as u32), (1, 0)));
            }
        }

        match grid[i].last().unwrap() {
            '/' => {
                initial_beams.push(((grid[0].len() as u32 - 1, i as u32), (0, -1)));
            },
            '\\' => {
                initial_beams.push(((grid[0].len() as u32 - 1, i as u32), (0, 1)));
            },
            _ => {
                initial_beams.push(((grid[0].len() as u32 - 1, i as u32), (-1, 0)));
            }
        }
    }

    for initial_beam in initial_beams {
        beams.clear();
        explored_beams.clear();
        energized_grid = grid.clone();
        beams.push(initial_beam);
        explored_beams.insert(initial_beam);
        energized_grid[initial_beam.0.1 as usize][initial_beam.0.0 as usize] = '#';

        while !beams.is_empty() {
            let mut new_beams: Vec<Beam> = Vec::new();

            for beam in beams.iter_mut() {
                // Extend the beam along the vector, until we hit a mirror or a wall
                match beam.1 {
                    (x, 0) => { // Beam is travelling on the x axis
                        let range: Vec<u32> = match x {
                            1 => (beam.0.0 + 1..grid[0].len() as u32).collect(),
                            -1 => (0..beam.0.0).rev().collect(),
                            _ => panic!("Invalid beam vector")
                        };

                        if range.is_empty() { // Beam has reached the end of the grid
                            continue;
                        }

                        for i in range {
                            match grid[beam.0.1 as usize][i as usize] {
                                '/' => {
                                    energized_grid[beam.0.1 as usize][i as usize] = '#';

                                    let new_vector = match x {
                                        1 => (0, 1),
                                        -1 => (0, -1),
                                        _ => panic!("Invalid beam vector")
                                    };

                                    let new_pos: BeamPos = ((i, beam.0.1), new_vector);
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = new_vector;
                                        beam.0.0 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(beam.clone());
                                    }
                                    break;
                                },
                                '\\' => {
                                    energized_grid[beam.0.1 as usize][i as usize] = '#';

                                    let new_vector = match x {
                                        1 => (0, -1),
                                        -1 => (0, 1),
                                        _ => panic!("Invalid beam vector")
                                    };

                                    let new_pos: BeamPos = ((i, beam.0.1), new_vector);
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = new_vector;
                                        beam.0.0 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(beam.clone());
                                    }
                                    break;
                                },
                                '.' | '-' => { // Energize this cell
                                    energized_grid[beam.0.1 as usize][i as usize] = '#';
                                },
                                '|' => { // Split the beam into 2
                                    energized_grid[beam.0.1 as usize][i as usize] = '#';

                                    let new_pos: BeamPos = ((i, beam.0.1), (0, 1));
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = (0, 1);
                                        beam.0.0 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(beam.clone());
                                    }

                                    let new_pos: BeamPos = ((i, beam.0.1), (0, -1));
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = (0, -1);
                                        beam.0.0 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(beam.clone());
                                    }
                                    break;
                                },
                                _ => { // Rest of the cases we do nothing
                                }
                            }
                        }
                    },
                    (0, y) => { // Beam is travelling on the y axis
                        let range: Vec<u32> = match y {
                            1 => (0..beam.0.1).rev().collect(),
                            -1 => (beam.0.1 + 1..grid.len() as u32).collect(),
                            _ => panic!("Invalid beam vector")
                        };

                        if range.is_empty() {
                            continue;
                        }

                        for i in range {
                            match grid[i as usize][beam.0.0 as usize] {
                                '/' => {
                                    energized_grid[i as usize][beam.0.0 as usize] = '#';

                                    let new_vector = match y {
                                        1 => (1, 0),
                                        -1 => (-1, 0),
                                        _ => panic!("Invalid beam vector")
                                    };

                                    let new_pos: BeamPos = ((beam.0.0, i), new_vector);
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = new_vector;
                                        beam.0.1 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(*beam);
                                    }
                                    break;
                                },
                                '\\' => {
                                    energized_grid[i as usize][beam.0.0 as usize] = '#';

                                    let new_vector = match y {
                                        1 => (-1, 0),
                                        -1 => (1, 0),
                                        _ => panic!("Invalid beam vector")
                                    };

                                    let new_pos: BeamPos = ((beam.0.0, i), new_vector);
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = new_vector;
                                        beam.0.1 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(*beam);
                                    }
                                    break;
                                },
                                '.' | '|' => { // Energize this cell
                                    energized_grid[i as usize][beam.0.0 as usize] = '#';
                                },
                                '-' => { // Split the beam into 2
                                    energized_grid[i as usize][beam.0.0 as usize] = '#';

                                    let new_pos: BeamPos = ((beam.0.0, i), (1, 0));
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = (1, 0);
                                        beam.0.1 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(beam.clone());
                                    }

                                    let new_pos: BeamPos = ((beam.0.0, i), (-1, 0));
                                    if !explored_beams.contains(&new_pos) {
                                        beam.1 = (-1, 0);
                                        beam.0.1 = i;
                                        explored_beams.insert((beam.0, beam.1));

                                        new_beams.push(*beam);
                                    }
                                    break;
                                },
                                _ => { // Rest of the cases we do nothing
                                }
                            }
                        }
                    },
                    _ => panic!("Invalid beam")
                }
            }

            beams = new_beams;
        }

        let result = energized_grid.iter().fold(0, |acc, row| acc + row.iter().filter(|c| **c == '#').count());
        if result > best {
            best = result;
        }
    }

    println!("Part 2: {}", best);
}