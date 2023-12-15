use std::{fs::read_to_string, collections::HashMap};

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
    let results: [&str; 2] = [ &lines[0], &lines[1]];
    let result = results
        .map(| line | line.split_once(':').unwrap().1.split_whitespace().collect::<Vec<_>>());
    let races: HashMap<u32, u32> = std::iter::zip(&result[0], &result[1])
        .into_iter()
        .map(| (value1, value2)| (value1.parse::<u32>().unwrap(), value2.parse::<u32>().unwrap()))
        .collect();
    let winning_results: Vec<u32> = races.into_iter().map(
        | (time, distance) |
        {
            let mut winning_ways = 0;
            for i in 0..=time {
                if i*(time-i) > distance {
                    winning_ways += 1
                }
            }
            winning_ways
        }
    ).collect();
    return winning_results.iter().fold(1, | sum, num | sum*num);
}

fn part_two(lines: &Vec<String>) -> u64 {
    let results: [&str; 2] = [ &lines[0], &lines[1]];
    let result = results
        .map(| line | line.split_once(':').unwrap().1.split_whitespace().collect::<Vec<_>>().join(""));

    let (time, distance): (u64, u64) = (result[0].parse::<u64>().unwrap(), result[1].parse::<u64>().unwrap());
    let mut winning_ways = 0;
    for i in 0..=time {
        if i*(time-i) > distance {
            winning_ways += 1
        }
    }
    return winning_ways;
}
