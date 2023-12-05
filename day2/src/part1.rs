use crate::utils::get_lines;

struct Game {
    id: u32,
    is_valid: bool
}

struct CubeLimit {
    red: u32,
    green: u32,
    blue: u32
}

fn parse_game(line: &str, maximum: &CubeLimit) -> Game {
    let mut game = Game {
        id: 0,
        is_valid: true
    };
    let parsed_line = line.replace(":", "").replace(";", "").replace(",", "");
    let splitted_line = parsed_line.split(" ").collect::<Vec<&str>>();
    game.id = splitted_line[1].parse::<u32>().unwrap();
    let rest = splitted_line[2..].chunks(2);
    for cube in rest {
        let amount = cube[0].parse::<u32>().unwrap();
        let color = cube[1];
        match color {
            "red" => if amount > maximum.red { game.is_valid = false; break; },
            "green" => if amount > maximum.green { game.is_valid = false; break; },
            "blue" => if amount > maximum.blue { game.is_valid = false; break; },
            _ => {}
        }
    }
    game
}

pub fn solve() {
    let cube_limit = CubeLimit {
        red: 12,
        green: 13,
        blue: 14
    };
    match get_lines("./input.txt") {
        Ok(lines) => {
            let mut sum = 0;
            for line in &lines {
                let game = parse_game(line.as_str(), &cube_limit);
                if game.is_valid {
                    sum += game.id;
                }
            }
            println!("{sum}");
        },
        Err(_) => {}
    };
}