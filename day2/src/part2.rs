use crate::utils::get_lines;

struct Game {
    id: u32,
    cubes: Cubes
}

struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

fn parse_game(line: &str) -> Game {
    let mut game = Game {
        id: 0,
        cubes: Cubes { 
            red: 0, 
            green: 0, 
            blue: 0 
        }
    };
    let parsed_line = line.replace(":", "").replace(";", "").replace(",", "");
    let splitted_line = parsed_line.split(" ").collect::<Vec<&str>>();
    game.id = splitted_line[1].parse::<u32>().unwrap();
    let rest = splitted_line[2..].chunks(2);
    for cube in rest {
        let amount = cube[0].parse::<u32>().unwrap();
        let color = cube[1];
        match color {
            "red" => if amount > game.cubes.red { game.cubes.red = amount; },
            "green" => if amount > game.cubes.green { game.cubes.green = amount; },
            "blue" => if amount > game.cubes.blue { game.cubes.blue = amount; },
            _ => {}
        }
    }
    game
}

pub fn solve() {
    match get_lines("./input.txt") {
        Ok(lines) => {
            let mut sum = 0;
            for line in &lines {
                let game = parse_game(line.as_str());
                let power = game.cubes.red * game.cubes.green * game.cubes.blue;
                sum += power;
            }
            println!("{sum}");
        },
        Err(_) => {}
    };
}