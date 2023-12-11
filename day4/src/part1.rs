use crate::utils::get_lines;

struct Card {
    id: u32,
    worth: u32
}

fn parse_card(line: String) -> Card {
    let mut card = Card {
        id: 0,
        worth: 0
    };
    let parsed_line = line.replace(":", "").replace("  ", " ").replace("  ", " ");
    let splitted_line = parsed_line.as_str().split(" ").collect::<Vec<&str>>();
    card.id = splitted_line[1].parse::<u32>().unwrap();
    let split_index = splitted_line.iter().position(|&c| c == "|").unwrap();
    let winner_numbers = &splitted_line[2..split_index];
    let card_numbers = &splitted_line[split_index+1..splitted_line.len()];
    for number in card_numbers {
        if winner_numbers.contains(number) {
            if card.worth == 0 {
                card.worth = 1;
                continue;
            }
            card.worth *= 2;
        }
    }
    card
}

pub fn solve() {
    match get_lines("./input.txt") {
        Ok(lines) => {
            let mut sum = 0;
            for line in lines {
                let card = parse_card(line);
                sum += card.worth;
            }
            println!("{}", sum);
        },
        Err(_) => {}
    };
}