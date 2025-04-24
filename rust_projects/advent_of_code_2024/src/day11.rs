use std::collections::HashMap;
fn main() {
    let lines = std::fs::read_to_string("data/day11.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(input: &str) -> i64 {
    let mut v: HashMap<i64, i64> = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| (x.parse().unwrap(), 1))
        .collect();

    for _ in 0..25 {
        let mut res = HashMap::new();
        for (x, count) in v {
            if x == 0 {
                *res.entry(1).or_insert(0) += count;
            } else if x.to_string().len() % 2 == 0 {
                let digits = x.to_string();
                let mid = digits.len() / 2;
                let left = digits[..mid].parse::<i64>().unwrap();
                let right = digits[mid..].parse::<i64>().unwrap();
                *res.entry(left).or_insert(0) += count;
                *res.entry(right).or_insert(0) += count;
            } else {
                *res.entry(x * 2024).or_insert(0) += count;
            }
        }
        v = res;
    }
    v.values().sum()
}

fn part2(input: &str) -> i64 {
    let mut v: HashMap<i64, i64> = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| (x.parse().unwrap(), 1))
        .collect();

    for _ in 0..75 {
        let mut res = HashMap::new();
        for (x, count) in v {
            if x == 0 {
                *res.entry(1).or_insert(0) += count;
            } else if x.to_string().len() % 2 == 0 {
                let digits = x.to_string();
                let mid = digits.len() / 2;
                let left = digits[..mid].parse::<i64>().unwrap();
                let right = digits[mid..].parse::<i64>().unwrap();
                *res.entry(left).or_insert(0) += count;
                *res.entry(right).or_insert(0) += count;
            } else {
                *res.entry(x * 2024).or_insert(0) += count;
            }
        }
        v = res;
    }
    v.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "125 17";
        assert_eq!(part1(report), 55312);
    }
}
