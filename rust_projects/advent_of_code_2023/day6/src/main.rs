use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    let time_distance: Vec<_> = input
        .lines()
        .map(|line| {
            let raw: Vec<_> = line.split(":").collect();
            raw.last()
                .expect("Error")
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let (time, distance) = (time_distance[0].to_vec(), time_distance[1].to_vec());
    time.iter()
        .enumerate()
        .map(|(i, x)| {
            let mut res = 0;
            for j in 0..*x {
                if (x - j) * j > distance[i] {
                    res += 1;
                }
            }
            res
        })
        .product::<u32>()
}

pub fn part2(input: &str) -> u64 {
    let time_distance: Vec<u64> = input
        .lines()
        .map(|line| {
            let raw: Vec<_> = line.split(":").collect();
            raw.last()
                .expect("Error")
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| c.to_digit(10).unwrap() as u64)
                .fold(0, |acc, elem| acc * 10 + elem)
        })
        .collect();
    let (time, distance) = (time_distance[0], time_distance[1]);
    let (mut first, mut last) = (0, 0);
    for j in 0..time / 2 {
        if (time - j) * j > distance {
            first = j;
            last = time - j;
            break;
        }
    }
    (last - first) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
