use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("input.txt");
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn part_one(lines: &Vec<String>) -> i32 {
    let green_cubes: i32 = 13;
    let red_cubes: i32 = 12;
    let blue_cubes: i32 = 14;
    let mut total_sum: i32 = 0;
    for line in lines {
        let mut possible: bool = true;
        let collection: Vec<&str> = line.split(':').collect::<Vec<&str>>();
        let game_name: Vec<&str>  = collection[0].split(' ').collect();
        let game_id: i32 = game_name[1].parse::<i32>().unwrap();
        let cube_handfuls: Vec<&str> = collection[1].split(';').collect();
        for handful in cube_handfuls {
            let cube_colours: Vec<&str> = handful.split(',').into_iter().map(|cube: &str| { cube.trim()}).collect();
            for cube in cube_colours {
                let cube_parts: Vec<&str> = cube.split(' ').collect();
                let amount: i32 = cube_parts[0].parse::<i32>().unwrap();
                let colour: &str = cube_parts[1];
                match colour {
                    "red" => if amount > red_cubes {
                        possible = false;
                    },
                    "green" => if amount > green_cubes {
                        possible = false;
                    },
                    "blue" => if amount > blue_cubes {
                        possible = false;
                    },
                    _ => println!("No such colour!"),
                }
            }
        }
        if possible {
            total_sum += game_id
        }
    }
    return total_sum;
}

fn part_two(lines: &Vec<String>) -> i32 {
    let mut total_sum: i32 = 0;
    for line in lines {
        let mut min_green_cubes: i32 = 0;
        let mut min_red_cubes: i32 = 0;
        let mut min_blue_cubes: i32 = 0;
        let collection = line.split(':').collect::<Vec<&str>>();
        let cube_handfuls: Vec<&str> = collection[1].split(';').collect();
        for handful in cube_handfuls {
            let cube_colours: Vec<&str> = handful.split(',').into_iter().map(|cube: &str| { cube.trim()}).collect();
            for cube in cube_colours {
                let cube_parts: Vec<&str> = cube.split(' ').collect();
                let amount: i32 = cube_parts[0].parse::<i32>().unwrap();
                let colour: &str = cube_parts[1];
                match colour {
                    "red" => if amount > min_red_cubes {
                       min_red_cubes = amount;
                    },
                    "green" => if amount > min_green_cubes {
                        min_green_cubes = amount;
                    },
                    "blue" => if amount > min_blue_cubes {
                        min_blue_cubes = amount;
                    },
                    _ => println!("No such colour!"),
                }
            }
        }
        total_sum += min_blue_cubes * min_red_cubes * min_green_cubes;
    }
    return total_sum;
}