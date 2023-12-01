#![allow(unused)]
use std::fs::File;
use std::io::Read;

fn main() {
    part_two();
}

fn read_file() -> Vec<String> {
    let mut file = File::open("src/input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}

fn part_one() {
    let mut sum = 0;
    for line in read_file() {
        let mut first_num = '0';

        for char in line.chars() {
            if char.is_digit(10) {
                first_num = char;
                break;
            }
        };

        let mut second_num = '0';

        for char in line.chars().rev() {
            if char.is_digit(10) {
                second_num = char;
                break;
            }
        };

        sum += format!("{}{}", first_num, second_num).parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}

fn part_two() {
    let mut file_contents = read_file();

    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    for i in 0..file_contents.len() {
        let mut line = file_contents[i].clone();

        let mut results = [i32::MAX; 9];
        let mut results_reversed = [i32::MIN; 9];

        for j in 0..digits.len() {
            let digit = digits[j];

            if let Some(pos) = line.find(digit) {
                results[j] = pos as i32;
            }

            if let Some(pos) = line.rfind(digit) {
                results_reversed[j] = pos as i32;
            }
        }

        if results.iter().all(|&x| x == i32::MAX) == false {
            let min_index = results.iter().position(|&x| x == *results.iter().min().unwrap()).unwrap();
            let max_index = results_reversed.iter().position(|&x| x == *results_reversed.iter().max().unwrap()).unwrap();

            if results[min_index] != results_reversed[max_index] {
                let mut result = String::new();

                result.push_str(&line[0..results[min_index] as usize]);
                result.push_str(&(min_index + 1).to_string());

                // This middle part doesnt need to be added if the two digits overlap
                if (results[min_index] as usize + digits[min_index].len()) < (results_reversed[max_index] as usize) {
                    result.push_str(&line[results[min_index] as usize + digits[min_index].len()..results_reversed[max_index] as usize]);
                }
                result.push_str(&(max_index + 1).to_string());
                result.push_str(&line[results_reversed[max_index] as usize + digits[max_index].len()..]);

                line = result;
            } else {
                line = line.replacen(digits[min_index], &(min_index + 1).to_string(), 1);
            }
        }

        file_contents[i] = line;
    }

    let mut sum = 0;
    for line in file_contents {
        let mut first_num = '0';

        for char in line.chars() {
            if char.is_digit(10) {
                first_num = char;
                break;
            }
        };

        let mut second_num = '0';

        for char in line.chars().rev() {
            if char.is_digit(10) {
                second_num = char;
                break;
            }
        };

        sum += format!("{}{}", first_num, second_num).parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}