use std::{fs, task::Wake};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let id_games: Vec<_> = line.split(":").collect();
            let id: Vec<_> = id_games
                .first()
                .expect("error getting game")
                .split(" ")
                .collect();
            let id = id
                .last()
                .expect("Error getting id")
                .parse::<u32>()
                .expect("Error parsing");
            let games: Vec<_> = id_games
                .last()
                .expect("Error extracting games")
                .split(";")
                .collect();
            for game in games {
                let parts: Vec<_> = game.trim().split(",").collect();
                for part in parts {
                    let color_val: Vec<_> = part.trim().split(" ").collect();
                    match (color_val.first(), color_val.last()) {
                        (Some(val), Some(color)) => match *color {
                            "blue" => {
                                if val.parse::<i32>().expect("error") > 14 {
                                    return 0;
                                }
                            }
                            "red" => {
                                if val.parse::<i32>().expect("error") > 12 {
                                    return 0;
                                }
                            }
                            "green" => {
                                if val.parse::<i32>().expect("error") > 13 {
                                    return 0;
                                }
                            }
                            _ => return 0,
                        },
                        _ => return 0,
                    }
                }
            }
            return id;
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let id_games: Vec<_> = line.split(":").collect();
            let games: Vec<_> = id_games
                .last()
                .expect("Error extracting games")
                .split(";")
                .collect();
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for game in games {
                let parts: Vec<_> = game.trim().split(",").collect();
                for part in parts {
                    let color_val: Vec<_> = part.trim().split(" ").collect();
                    match (color_val.first(), color_val.last()) {
                        (Some(val), Some(color)) => {
                            let val = val.parse::<u32>().expect("error");
                            match *color {
                                "blue" => {
                                    if blue < val {
                                        blue = val;
                                    }
                                }
                                "red" => {
                                    if red < val {
                                        red = val;
                                    }
                                }
                                "green" => {
                                    if green < val {
                                        green = val;
                                    }
                                }
                                _ => return 0,
                            }
                        }
                        _ => return 0,
                    }
                }
            }
            return red * blue * green;
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part2(input);
        assert_eq!(result, 2286);
    }
}
