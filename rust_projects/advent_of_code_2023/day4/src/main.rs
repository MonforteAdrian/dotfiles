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
            let card: Vec<_> = line.split(":").collect();
            let numbers: Vec<_> = card.last().expect("Error").trim().split("|").collect();
            let winners: Vec<_> = numbers
                .first()
                .expect("Error")
                .trim()
                .split_whitespace()
                .collect();
            let yours: Vec<_> = numbers
                .last()
                .expect("Error")
                .trim()
                .split_whitespace()
                .collect();
            let mut res = 0;
            for your in yours {
                if winners.contains(&your) {
                    if res == 0 {
                        res = 1;
                    } else {
                        res = res * 2;
                    }
                }
            }
            res
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    let mut res: u32 = 0;
    let mut v: Vec<i32> = vec![0; 15];
    let lines: Vec<_> = input.lines().collect();
    for line in lines {
        let card: Vec<_> = line.split(":").collect();
        let numbers: Vec<_> = card.last().expect("Error").trim().split("|").collect();
        let winners: Vec<_> = numbers
            .first()
            .expect("Error")
            .trim()
            .split_whitespace()
            .collect();
        let yours: Vec<_> = numbers
            .last()
            .expect("Error")
            .trim()
            .split_whitespace()
            .collect();
        let mut count = 0;
        for your in yours {
            if winners.contains(&your) {
                count += 1;
            }
        }
        for _ in 0..=v[0] {
            for i in 1..=count {
                v[i] += 1;
            }
            res += 1;
        }
        v.remove(0);
        v.push(0);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
