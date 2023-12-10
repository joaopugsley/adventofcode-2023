use crate::utils::get_lines;
use std::collections::HashMap;

struct Position(usize, usize);

fn create_range(start: usize, end: usize) -> std::ops::RangeInclusive<usize> {
    if start == 0 {
        return start..=end + 1;
    }
    (start - 1)..=end + 1
}

fn is_adjacent(number_pos: &(Position, Position), gear_pos: &Position) -> bool {
    let (start_pos, end_pos) = number_pos;
    let x_range = create_range(start_pos.0, end_pos.0);
    let y_range = create_range(start_pos.1, end_pos.1);
    if x_range.contains(&gear_pos.0) && y_range.contains(&gear_pos.1) {
        return true;
    }
    false
}

struct Symbols(Vec<Position>);
struct Numbers(HashMap<usize, Vec<(Position, Position)>>);
struct Data {
    symbols: Symbols,
    numbers: Numbers
}

fn parse_data(input: &Vec<String>) -> Data {
    let mut symbols: Vec<Position> = Vec::new();
    let mut numbers: HashMap<usize, Vec<(Position, Position)>> = HashMap::new();
    for (row, line) in input.into_iter().enumerate() {
        let mut col: usize = 0;
        let chars: Vec<char> = line.chars().collect();
        while col < line.len() {
            if chars[col] == '*' {
                symbols.push(Position(row, col));
            } else if chars[col].is_numeric() {
                let end_col = chars[col..].iter().position(|&c| !c.is_numeric()).unwrap_or(line.len() - col);
                let end_col = col + end_col - 1;
                let number: usize = chars[col..end_col + 1].iter().collect::<String>().parse::<usize>().unwrap();
                numbers.entry(number).or_insert(Vec::new()).push((Position(row, col), Position(row, end_col)));
                col = end_col;
            }
            col += 1;
        }
    }
    Data {
        symbols: Symbols(symbols),
        numbers: Numbers(numbers)
    }
}

pub fn solve() {
    match get_lines("./input.txt") {
        Ok(lines) => {
            let data = parse_data(&lines);
            let mut sum = 0;
            for gear_pos in &data.symbols.0 {
                let mut gears: Vec<usize> = Vec::new();
                for (number, positions) in &data.numbers.0 {
                    for pos in positions {
                        if is_adjacent(&pos, &gear_pos) {
                            gears.push(*number);
                        }
                    }
                }
                if gears.len() >= 2 {
                    sum += gears.iter().product::<usize>();
                }
            }
            println!("{sum}");
        },
        Err(_) => {}
    }
}
