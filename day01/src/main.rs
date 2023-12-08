use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let lines: Vec<String> = read_lines("input.txt");
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> i32 {
    let mut total_sum: i32 = 0;
    for line in lines {
        let mut line_nums: Vec<char> = ['\0', '\0'].to_vec();
        for char in line.chars() {
            if char.is_digit(10) {
                if line_nums[0] == '\0' {
                    line_nums[0] = char;
                }
                else {
                    line_nums[1] = char;
                }
            } 
        }
        if line_nums[1] == '\0' {
            line_nums[1] = line_nums[0]
        }
        total_sum += (line_nums.into_iter().collect::<String>()).parse::<i32>().unwrap()
    }
    return total_sum;
}

fn part_two(lines: &Vec<String>) -> i32 {
    let char_regexes: [(Regex, usize, char); 11] = [
        (Regex::new(r"\d....").unwrap(), 1, 'a'),
        (Regex::new(r"six..").unwrap(), 2, '6'),
        (Regex::new(r"one..").unwrap(), 2, '1'),
        (Regex::new(r"two..").unwrap(), 2, '2'),
        (Regex::new(r"zero.").unwrap(), 2, '0'),
        (Regex::new(r"four.").unwrap(), 3, '4'),
        (Regex::new(r"five.").unwrap(), 3, '5'),
        (Regex::new(r"nine.").unwrap(), 3, '9'),
        (Regex::new(r"seven").unwrap(), 4, '7'),
        (Regex::new(r"eight").unwrap(), 4, '8'),
        (Regex::new(r"three").unwrap(), 4, '3'),
        ];
    let mut total_sum:i32 = 0;
    for line in lines {
        let usableline = format!("{:a<60}", line);
        let mut line_nums: Vec<char> = ['\0', '\0'].to_vec();
        let inter = usableline.as_bytes();
        let mut windows = inter.windows(5);
        while let Some(win) = windows.next() {
            let window = String::from_utf8_lossy(win).to_string();
            for regex in &char_regexes {
                if regex.0.is_match(&window) {
                    if line_nums[0] == '\0' {
                        if regex.2 == 'a' {
                            line_nums[0] = window.chars().nth(0).unwrap();
                        }
                        else {
                            line_nums[0] = regex.2;
                        }
                    }
                    else {
                        if regex.2 == 'a' {
                            line_nums[1] = window.chars().nth(0).unwrap();
                        }
                        else {
                            line_nums[1] = regex.2;
                        }
                    }
                }
            }
        }
        if line_nums[1] == '\0' {
            line_nums[1] = line_nums[0]
        }
        total_sum += (line_nums.iter().collect::<String>()).parse::<i32>().unwrap();
    }
    return total_sum;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}