use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines: Vec<String> = read_lines("input.txt");
    println!("Part One: {}", part_one(&lines));
}


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn part_one(lines: &Vec<String>) -> u64 {
    let hands: HashMap<&str, u64> = [("FiveOfAKind", 70000000000), ("FourOfAKind", 60000000000), ("FullHouse", 50000000000), ("ThreeOfAKind", 40000000000), ("TwoPair", 30000000000), ("OnePair", 20000000000), ("HighCard", 10000000000)].into_iter().collect();
    let mut ranked: Vec<(u32, u64)> = Vec::new();
    let mut total_bid: u64 = 0;
    for line in lines {
        let hand: Vec<&str> = line.split_whitespace().collect();
        let mut hand_value = 0;
        // Get the cardinality of each card in the hand
        let letter_counts: HashMap<char, i32> =
        hand[0]
            .to_lowercase()
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
        let chars: Vec<(char, i32)> = letter_counts.into_iter().collect();

        // Evaluate the value of the hand
        if chars.iter().any(| char | char.1 == 5) {
            hand_value = *hands.get("FiveOfAKind").unwrap();
        }

        else if chars.iter().any(| char | char.1 == 4) {
            hand_value = *hands.get("FourOfAKind").unwrap();
        }

        else if chars.iter().any(| char | char.1 == 3) & chars.iter().any(| char | char.1 == 2) {
            hand_value = *hands.get("FullHouse").unwrap();
        }

        else if chars.iter().any(| char | char.1 == 3) {
            hand_value = *hands.get("ThreeOfAKind").unwrap();
        }

        else if chars.iter().filter(| char | char.1 == 2).count() == 2 {
            hand_value = *hands.get("TwoPair").unwrap();
        }

        else if chars.iter().any(| char | char.1 == 2) {
            hand_value = *hands.get("OnePair").unwrap();
        }

        else if chars.iter().all(| char | char.1 == 1) {
            hand_value = *hands.get("HighCard").unwrap();
        }
        
        hand_value += get_hand_value(hand[0]);

        ranked.push((hand[1].parse::<u32>().unwrap(), hand_value))
    }
    // Sort hands in ascending order by hand value
    ranked.sort_by_key(| hand | hand.1);
    let mut rank: u32 = 1;
    for hand in ranked {
        total_bid += (hand.0 * rank) as u64;
        rank += 1;
    }


    return total_bid;
}

fn get_hand_value(hand: &str) -> u64 {
    let mut value = 0;
    let mut multiplier: u64 = 100000000;
    let order: HashMap<char, u64> = [('A', 13), ('K', 12), ('Q', 11), ('J', 10), ('T', 9), ('9', 8), ('8', 7), ('7', 6), ('6', 5), ('5', 4), ('4', 3), ('3', 2), ('2', 1)].into_iter().collect();
    for char in hand.chars() { 
        value += order.get(&char).unwrap() * multiplier;
        multiplier /= 100;
    }
    return  value;
}