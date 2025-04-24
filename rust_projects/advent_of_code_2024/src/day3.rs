use regex::Regex;
use std::fs;

fn main() {
    let mut res1 = 0;
    let mut res2 = 0;
    let lines = fs::read_to_string("data/day3.txt").unwrap();
    for line in lines.lines() {
        res1 += part1(line);
    }
    res2 += part2(&lines);

    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(s: &str) -> usize {
    let patt = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    patt.captures_iter(s)
        .map(|capture| {
            let a: usize = capture.get(1).unwrap().as_str().parse().unwrap();
            let b: usize = capture.get(2).unwrap().as_str().parse().unwrap();
            a * b
        })
        .sum()
}

fn part2(s: &str) -> i32 {
    let do_patt = Regex::new(r"do\(\)").unwrap();
    let dont_patt = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true;
    let mut total = 0;

    let all_patt = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for caps in all_patt.captures_iter(s) {
        dbg!(&caps[0]);
        if do_patt.find(&caps[0]).is_some() {
            enabled = true;
        } else if dont_patt.find(&caps[0]).is_some() {
            enabled = false;
        } else if enabled {
            if let Some(x) = caps.get(1) {
                if let Some(y) = caps.get(2) {
                    let x: i32 = x.as_str().parse().unwrap();
                    let y: i32 = y.as_str().parse().unwrap();
                    total += x * y;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "mul(2,3)mul(4,5)";
        assert_eq!(part1(report), 26);
        let report = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(report), 161);
        let report = "mul(,5)mul(a,b)xmulmulmul)5,6(";
        assert_eq!(part1(report), 0);
        let report = "mul(0,0)mul(1,999)mul(123,456)";
        assert_eq!(part1(report), 57087);
        let report = "@@mul(12,34)fake_mul(56,78)real_mul(90,12)+mul(7,8)-junk(mul(5,6))mul(2,3)mul(100,400)!!";
        assert_eq!(part1(report), 45948);
    }

    #[test]
    fn test_part2() {
        let report = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(report), 48);
        let report = "mul(2,3)do()mul(4,5)don't()mul(6,7)";
        assert_eq!(part2(report), 26);
        let report = "don't()mul(3,4)mul(5,6)mul(7,8)";
        assert_eq!(part2(report), 0);
        let report = "mul(10,10)don't()mul(20,20)do()mul(5,5)";
        assert_eq!(part2(report), 125);
        let report = "do()mul(2,2)don't()mul(3,3)do()mul(4,4)don't()mul(5,5)";
        assert_eq!(part2(report), 20);
        let report = "junk(do())more_junk(don't())even_more_junk";
        assert_eq!(part2(report), 0);
        let report =
            "mul(1,1)don't()do()mul(2,2)don't()mul(3,3)do()mul(4,4)mul(5,5)don't()mul(6,6)";
        assert_eq!(part2(report), 46);
        let report = "do()mul(999,999)don't()mul(123,456)do()mul(100,100)";
        assert_eq!(part2(report), 1008001);
    }
}
