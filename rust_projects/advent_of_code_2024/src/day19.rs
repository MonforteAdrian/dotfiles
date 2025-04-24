fn main() {
    let input = std::fs::read_to_string("data/day19.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn possible(towels: Vec<&str>, design: &str) -> bool {
    // Helper function for recursion
    fn can_construct(
        towels: &Vec<&str>,
        design: &str,
        memo: &mut std::collections::HashMap<String, bool>,
    ) -> bool {
        if design.is_empty() {
            return true;
        }
        if let Some(&result) = memo.get(design) {
            return result;
        }
        for &towel in towels {
            if let Some(stripped) = design.strip_prefix(towel) {
                if can_construct(towels, stripped, memo) {
                    memo.insert(design.to_string(), true);
                    return true;
                }
            }
        }
        memo.insert(design.to_string(), false);
        false
    }

    let mut memo = std::collections::HashMap::new();
    can_construct(&towels, design, &mut memo)
}

fn part1(s: &str) -> usize {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let towels: Vec<&str> = parts[0].split(", ").collect();
    let designs: Vec<&str> = parts[1].lines().collect();
    let mut res = 0;
    for design in designs {
        if possible(towels.clone(), design) {
            res += 1;
        }
    }
    res
}

fn count_arrangements(towels: &Vec<&str>, design: &str) -> usize {
    // Helper function for recursion
    fn count_construct(
        towels: &Vec<&str>,
        design: &str,
        memo: &mut std::collections::HashMap<String, usize>,
    ) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(&count) = memo.get(design) {
            return count;
        }
        let mut total = 0;
        for &towel in towels {
            if let Some(stripped) = design.strip_prefix(towel) {
                total += count_construct(towels, stripped, memo);
            }
        }
        memo.insert(design.to_string(), total);
        total
    }

    let mut memo = std::collections::HashMap::new();
    count_construct(towels, design, &mut memo)
}
fn part2(s: &str) -> usize {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let towels: Vec<&str> = parts[0].split(", ").collect();
    let designs: Vec<&str> = parts[1].lines().collect();
    let mut total_arrangements = 0;
    for design in designs {
        total_arrangements += count_arrangements(&towels, design);
    }
    total_arrangements
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(part1(report), 6);
    }
}
