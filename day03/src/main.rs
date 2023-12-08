use std::fs::read_to_string;

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

fn part_one(lines: &Vec<String>) -> i32 {
    let mut symbols: [(i32, i32); 1000] = std::array::from_fn(|_| (0, 0));
    let mut symbol_index: usize = 0;
    let mut numbers: [(String, i32, i32); 2000] = std::array::from_fn(|_| (String::new(), 0, 0)); 
    let mut number_index: usize = 0;
    let mut adjacent_numbers: [i32; 2000] = std::array::from_fn(|_| 0,);
    let mut adjacent_index: usize = 0;
    let mut line_index: i32 = 0;
    for line in lines {
        let mut char_index: i32 = 0;
        for char in line.chars() {
            // We found a period, if this is the first period after a number, move the number index ahead
            if (char == '.') & ((numbers[number_index].1 != 0) | (numbers[number_index].2 != 0)) {
                number_index += 1;
            }
            if !char.is_alphanumeric() {
                // We found a symbol, if it's not a period add its location to the array. We should also move the number index ahead if there is a number there
                if char != '.' {
                    if (numbers[number_index].1 != 0) | (numbers[number_index].2 != 0) {
                        number_index += 1;
                    }
                    symbols[symbol_index] = (line_index, char_index);
                    symbol_index += 1;
                }
            }
            else if char.is_digit(10) {
                // If there is no number at index, initialize it
                if (numbers[number_index].1 == 0) & (numbers[number_index].2 == 0) {
                    numbers[number_index] = (format!("{}", char), line_index, char_index);
                }
                // If there is, just add to it
                else {
                    numbers[number_index] = (format!("{}{}", numbers[number_index].0, char), numbers[number_index].1, numbers[number_index].2);
                }
                
            }
            char_index += 1;
        }
        line_index += 1;
    }
    // We have gathered locations of all symbols and numbers, now check which are adjacent numbers
    for symbol in symbols {
        if (symbol.0 != 0) & (symbol.1 != 0) {
            for number in numbers.clone() {
                // If it is an adjacent number, add it to the list
                if (symbol.0.abs_diff(number.1) < 2) & ((symbol.1.abs_diff(number.2) < 2) | (symbol.1.abs_diff(number.2+(number.0.len() as i32)-1) < 2)) & ((number.1 != 0) | (number.2 != 0)) {
                    adjacent_numbers[adjacent_index] = number.0.parse::<i32>().unwrap();
                    adjacent_index += 1;
                }
            }
        }
    }
    return adjacent_numbers.iter().sum();
}
