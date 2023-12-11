use crate::utils::get_lines;

#[derive(Clone)]
struct Card {
    id: u32,
    matches: u32
}

fn parse_card(line: String) -> Card {
    let mut card = Card {
        id: 0,
        matches: 0
    };
    let parsed_line = line.replace(":", "").replace("  ", " ").replace("  ", " ");
    let splitted_line = parsed_line.as_str().split(" ").collect::<Vec<&str>>();
    card.id = splitted_line[1].parse::<u32>().unwrap();
    let split_index = splitted_line.iter().position(|&c| c == "|").unwrap();
    let winner_numbers = &splitted_line[2..split_index];
    let card_numbers = &splitted_line[split_index+1..splitted_line.len()];
    for number in card_numbers {
        if winner_numbers.contains(number) {
            card.matches += 1;
        }
    }
    card
}

pub fn solve() {
    match get_lines("./input.txt") {
        Ok(lines) => {
            let mut cards: Vec<Card> = Vec::new();
            for line in lines {
                let card = parse_card(line);
                cards.push(card);
            }

            let mut new_cards: Vec<u32> = Vec::new();
            for card in &cards {
                new_cards.push(card.id);
            }

            let mut i = 0;
            while i < new_cards.len() {
                let card_id = new_cards[i] - 1;
                if let Some(card) = cards.get(card_id as usize) {
                    for id in card.id + 1..=card.id + card.matches {
                        new_cards.push(id);
                    }
                }
                i += 1;
            }

            println!("{}", new_cards.len());
        },
        Err(_) => {}
    };
}