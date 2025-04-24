use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("data/day22.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn sequence(mut sn: i64) -> i64 {
    // First we need to find the pattern
    // Create qeue where we have the last 4 diff and if not remove older and add new
    for _ in 0..2000 {
        sn ^= sn * 64;
        sn %= 16777216;
        sn ^= sn / 32;
        sn %= 16777216;
        sn ^= sn * 2048;
        sn %= 16777216;
        // extract the last digit
        // check with the previous one the diff
        // check if the pattern is the desired
        // return the value after the pattern
    }
    0
}

fn part1(s: &str) -> i64 {
    let secret_numbers: Vec<i64> = s.lines().map(|line| line.parse().unwrap()).collect();
    secret_numbers.par_iter().map(|sn| sequence(*sn)).sum()
}

fn part2(s: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "";
        assert_eq!(part2(report), 0);
    }
}
