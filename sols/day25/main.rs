#![allow(unused)]

use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fs::File;
use std::io::Read;
use std::mem::swap;
use std::os::unix::fs::DirEntryExt;
use regex::Regex;
use rand::prelude::SliceRandom;

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
    // Failed attempt at Karger's algorithm
    let input = read_file();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    for line in input.iter() {
        let mut split = line.split(": ");
        let name = split.next().unwrap().to_string();
        let mut connections = split.next().unwrap().split(' ');

        if !vertices.iter().any(|n| *n == name) {
            vertices.push(name);
        }

        for con in connections {
            if !vertices.iter().any(|n| n == con) {
                vertices.push(con.to_string());
            }
        }
    }

    for line in input.iter() {
        let mut split = line.split(": ");
        let name = split.next().unwrap().to_string();
        let mut connections = split.next().unwrap().split(' ');
        let index = vertices.iter().position(|n| *n == name).unwrap();

        for con in connections {
            let index_child = vertices.iter().position(|n| *n == con).unwrap();
            let edge = if index < index_child {
                (index, index_child)
            } else {
                (index_child, index)
            };

            if !edges.contains(&edge) {
                edges.push(edge);
            }
        }
    }

    let mut contracted_vertices = vertices.clone();
    let mut contracted_edges = edges.clone();

    while contracted_edges.len() > 3 {
        let edge = *contracted_edges.choose(&mut rand::thread_rng()).unwrap();

        for edge2 in contracted_edges.iter_mut() {
            if edge != *edge2 {
                if edge2.0 == edge.1 {
                    edge2.0 = edge.0;
                } else if edge2.1 == edge.1 {
                    edge2.1 = edge.0;
                }

                if edge2.0 > edge2.1 {
                    swap(&mut edge2.0, &mut edge2.1);
                }
            }
        }

        contracted_edges.remove(contracted_edges.iter().position(|e| *e == *edge).unwrap());

        contracted_edges.retain(|e| e.0 != e.1);
    }

    for edge in contracted_edges.iter() {
        println!("{} {}", contracted_vertices[edge.0], contracted_vertices[edge.1]);
    }
}

fn part_two() {

}