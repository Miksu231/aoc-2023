
use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = read_lines("input.txt");
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn part_one(lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for card in lines {
        let mut card_wins = get_card_wins(card);
        if card_wins > 0 {
            card_wins = u32::pow(2, card_wins-1)
        }
        sum += card_wins;
    }
    return sum;
}

fn part_two(lines: &Vec<String>) -> u32 {
    let mut card_index = 0;
    let mut pile = HashMap::<usize, u32>::new();
    for card in lines {
        let card_wins = get_card_wins(card);
        let mut qty: u32 = 1;
        if let Some(quantity) = pile.get_mut(&card_index) {
            *quantity += 1;
            qty = *quantity;
        } else {
            pile.insert(card_index, qty);
        }

        if card_wins > 0 {
            for i in 1..=card_wins as usize {
                if let Some(q) = pile.get_mut(&(card_index + i)) {
                    *q += qty;
                } else {
                    pile.insert(card_index + i, qty);
                }
            }
        }
        card_index+=1;
    }
    return pile.into_values().sum();
}

fn get_card_wins(card: &str)-> u32 
{
    let numbers = card.split(':').collect::<Vec<&str>>()[1].split('|').collect::<Vec<&str>>();
    let winning_numbers: Vec<u32> = numbers[0].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect();
    let card_numbers: Vec<u32> = numbers[1].split_whitespace().map(|number| number.parse::<u32>().unwrap()).collect();
    let mut card_wins = 0; 
    for number in card_numbers {
        if winning_numbers.contains(&number) {
            card_wins += 1;
        }
    }
    return card_wins;
}
