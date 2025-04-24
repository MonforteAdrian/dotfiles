fn main() {
    let lines = std::fs::read_to_string("data/day7.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(s: &str) -> u64 {
    let mut res = 0;
    for line in s.lines() {
        let (test_value, values) = parse_line(line);
        let calibrated = check_operation(test_value, values);
        if calibrated {
            res += test_value;
        }
    }
    res
}

fn part2(s: &str) -> u64 {
    s.lines()
        .map(parse_line)
        .filter(|(test_value, values)| check_operation2(*test_value, values.clone()))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn check_operation(test_value: u64, values: Vec<i32>) -> bool {
    let spaces = values.len() - 1;
    let possibilites = 2_i32.pow(spaces as u32);
    let binary_max = spaces;
    for i in 0..possibilites {
        let bin: Vec<char> = format!("{i:0binary_max$b}").chars().rev().collect();
        let mut sum: u64 = values[0] as u64;
        for space in 0..spaces {
            if bin[space] == '0' {
                sum += values[space + 1] as u64;
            } else {
                sum *= values[space + 1] as u64;
            }
        }
        if test_value == sum {
            return true;
        }
    }
    false
}

fn check_operation_recursive(test_value: u64, values: &[u64], current: u64) -> bool {
    if values.is_empty() {
        return current == test_value;
    }
    let next = values[0];
    // Try addition
    if check_operation_recursive(test_value, &values[1..], current + next) {
        return true;
    }
    // Try multiplication
    if check_operation_recursive(test_value, &values[1..], current * next) {
        return true;
    }
    // Try concatenation
    let concatenated = current * 10u64.pow(next.to_string().len() as u32) + next;
    if check_operation_recursive(test_value, &values[1..], concatenated) {
        return true;
    }
    false
}

fn check_operation2(test_value: u64, values: Vec<i32>) -> bool {
    let values: Vec<u64> = values.into_iter().map(|v| v as u64).collect(); // Prevent overflow
    check_operation_recursive(test_value, &values[1..], values[0])
}

fn parse_line(s: &str) -> (u64, Vec<i32>) {
    let mut parts = s.split(':');
    let test_value = parts.next().unwrap().trim().parse().unwrap();
    let values = parts
        .next()
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    (test_value, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part1(report), 3749);
    }
}
