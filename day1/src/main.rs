use std::{fs::File, io::{self, BufReader, BufRead}, collections::HashMap};

fn get_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn extract_part1(line: &String) -> i32 {
    let mut first_number: u32 = 10;
    let mut last_number: u32 = 0;
    for char in line.chars() {
        if char.is_digit(10) {
            if first_number == 10 {
                if let Some(digit) = char.to_digit(10) {
                    first_number = digit;
                }
            }
            if let Some(digit) = char.to_digit(10) {
                last_number = digit;
            }
        }
    }
    if first_number == 10 {
        first_number = 0;
    }
    let mut num = String::new();
    num.push_str(first_number.to_string().as_str());
    num.push_str(last_number.to_string().as_str());

    num.parse::<i32>().unwrap()
}

fn extract_part2(line: &String) -> i32 {
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut first_number: u32 = 10;
    let mut last_number: u32 = 0;
    let mut ln = line.to_owned();
    for _ in line.chars() {
        let char = ln.chars().nth(0).unwrap();
        if char.is_digit(10) {
            if first_number == 10 {
                if let Some(digit) = char.to_digit(10) {
                    first_number = digit;
                }
            }
            if let Some(digit) = char.to_digit(10) {
                last_number = digit;
            }
        } else {
            for (key, value) in digit_map.iter() {
                if ln.starts_with(key) {
                    if first_number == 10 {
                        first_number = *value as u32;
                    }
                    last_number = *value as u32;
                }
            }
        }
        ln.remove(0);
    }
    if first_number == 10 {
        first_number = 0;
    }
    let mut num = String::new();
    num.push_str(first_number.to_string().as_str());
    num.push_str(last_number.to_string().as_str());

    num.parse::<i32>().unwrap()
}

fn solve() {
    match get_lines("./input.txt") {
        Ok(lines) => {
            let mut sum = 0;
            for line in &lines {
                let line_digits = extract_part2(line);
                sum += line_digits;
            }
            println!("{sum}");
        },
        Err(_) => {}
    };
}

fn main() {
    solve();
}
