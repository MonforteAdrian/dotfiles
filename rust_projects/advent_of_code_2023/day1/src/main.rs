use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|c| c.to_digit(10));
            match it.clone().last() {
                Some(y) => it.next().expect("error 1") * 10 + y,
                None => it.next().expect("error 1") * 10 + it.next().unwrap(),
            }
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(|line|{
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")})
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, 281);
    }
}
