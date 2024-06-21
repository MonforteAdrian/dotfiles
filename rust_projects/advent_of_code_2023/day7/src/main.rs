use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    let hand_bid: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    let (hand, bid, rank) = (
        hand_bid.iter().map(|x| x[0]).collect::<Vec<_>>(),
        hand_bid
            .iter()
            .map(|x| x[1].parse::<u32>().unwrap())
            .collect::<Vec<_>>(),
        vec![0; hand_bid.len()],
    );
    for (i, hand) in hand.iter().enumerate() {}

    println!("{:?} {:?} {:?}", hand, bid, rank);
    0
}

pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part1(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_test() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
