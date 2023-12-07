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
    let mut hands = Vec::new();
    for line in read_file() {
        let parts = line.split(' ').collect::<Vec<&str>>();

        hands.push((parts[0].to_string(), parts[1].parse::<i32>().unwrap()))
    }

    hands.sort_by(sorting_fn_a);

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        total += hand.1 * (i as i32 + 1);
    }

    println!("Part 1: {}", total);
}

fn part_two() {
    let mut hands = Vec::new();
    for line in read_file() {
        let parts = line.split(' ').collect::<Vec<&str>>();

        hands.push((parts[0].to_string(), parts[1].parse::<i32>().unwrap()))
    }

   for hand in &hands {
       println!("{}: {}", hand.0, type_of_hand_b(&hand.0));
   }

    hands.sort_by(sorting_fn_b);

    let mut total = 0;

    for (i, hand) in hands.iter().enumerate() {
        total += hand.1 * (i as i32 + 1);
    }

    println!("Part 2: {}", total);
}

fn sorting_fn_a(a: &(String, i32), b: &(String, i32)) -> Ordering {
    let strengths: HashMap<char, i32> = [
        ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6),
        ('7', 7), ('8', 8), ('9', 9), ('T', 10), ('J', 11),
        ('Q', 12), ('K', 13), ('A', 14)
    ].iter().cloned().collect();

    let type_a = type_of_hand_a(&a.0);
    let type_b = type_of_hand_a(&b.0);
    let chars_a = a.0.chars().collect::<Vec<char>>();
    let chars_b = b.0.chars().collect::<Vec<char>>();

    if type_a == type_b {
        // Check each hand card by card from left to right
        for i in 0..chars_a.len() {
            let card_a = chars_a[i];
            let card_b = chars_b[i];

            if strengths[&card_a] != strengths[&card_b] {
                return strengths[&card_a].cmp(&strengths[&card_b]);
            }
        }

        Ordering::Equal
    } else {
        type_a.cmp(&type_b)
    }
}

fn type_of_hand_a(hand: &str) -> i32 {
    let hand_set = hand.chars().collect::<HashSet<char>>();

    // Strength 6 is five of a kind and strength 0 is high card
    // Check for five of a kind
    if hand_set.len() == 1 {
        return 6;
    }

    // 4 of a kind and full house have the same card_set
    if hand_set.len() == 2 {
        // Check for 4 of a kind
        let count = hand.matches(*hand_set.iter().next().unwrap()).count();
        return if count == 4 || count == 1 {
            5
        } else {
            4
        }
    }

    // 3 of a kind and 2 pair have the same card_set
    if hand_set.len() == 3 {
        // Check for 3 of a kind by counting occurrences of each card
        for card in hand_set.iter() {
            let count = hand.matches(*card).count();
            if count == 3 {
                return 3;
            }
        }

        return 2;
    }

    // Check for one pair
    if hand_set.len() == 4 {
        1
    } else {
        0
    }
}


fn sorting_fn_b(a: &(String, i32), b: &(String, i32)) -> Ordering {
    let strengths: HashMap<char, i32> = [
        ('2', 2), ('3', 3), ('4', 4), ('5', 5), ('6', 6),
        ('7', 7), ('8', 8), ('9', 9), ('T', 10), ('J', 1),
        ('Q', 12), ('K', 13), ('A', 14)
    ].iter().cloned().collect();

    let type_a = type_of_hand_b(&a.0);
    let type_b = type_of_hand_b(&b.0);
    let chars_a = a.0.chars().collect::<Vec<char>>();
    let chars_b = b.0.chars().collect::<Vec<char>>();

    if type_a == type_b {
        // Check each hand card by card from left to right
        for i in 0..chars_a.len() {
            let card_a = chars_a[i];
            let card_b = chars_b[i];

            if strengths[&card_a] != strengths[&card_b] {
                return strengths[&card_a].cmp(&strengths[&card_b]);
            }
        }

        Ordering::Equal
    } else {
        type_a.cmp(&type_b)
    }
}

fn type_of_hand_b(hand: &str) -> i32 {
    // If no joker, same as part a
    if !hand.contains('J') {
        return type_of_hand_a(hand);
    }

    let mut hand_set = hand.chars().collect::<HashSet<char>>();

    if (hand_set.len() != 1) { // Remove the Joker if we have any other card also in the hand
        hand_set.remove(&'J');
    }

    let joker_count = hand.matches('J').count();

    // Strength 6 is five of a kind and strength 0 is high card
    // Check for five of a kind
    if joker_count == 5 || hand.matches(*hand_set.iter().next().unwrap()).count() + joker_count == 5 {
        return 6;
    }

    // With 3 jokers we can always make a four of a kind (if we had 4 jokers we would have 5 of a kind)
    if joker_count == 3 {
        return 5;
    }

    // 4 of a kind and full house have the same card_set
    if hand_set.len() == 2 {
        // Check if we can make 4 of a kind with any card
        for card in hand_set.iter() {
            let count = hand.matches(*card).count();
            if count + joker_count == 4 {
                return 5;
            }
        }

        return 4; // If we can't make 4 of a kind with any card, we have a full house
    }

    // With 2 jokers we can always make a three of a kind or with 1 joker and a pair
    if joker_count == 2 || hand_set.len() == 3 {
        3
    } else {
        1
    }
}