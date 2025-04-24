use std::fs;

fn main() {
    let mut v: Vec<&str> = Vec::new();
    let mut res1 = 0;
    let mut res2 = 0;
    for line in fs::read_to_string("data/day2.txt").unwrap().lines() {
        v = line.trim().split(" ").collect();
        //if part1(v.clone()) {
        //    res1 += 1;
        //}
        if part2(v) {
            res2 += 1;
        }
    }

    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(v: Vec<&str>) -> bool {
    if v.is_empty() || v.len() == 1 {
        return true;
    };
    if v[0].parse::<i32>().expect("error parsing") == v[1].parse::<i32>().expect("error parsing") {
        return false;
    }
    let inc = if v[0].parse::<i32>().expect("error parsing")
        > v[1].parse::<i32>().expect("error parsing")
    {
        1
    } else {
        -1
    }; // -1 dec 1 inc if 0 we return false
    for i in 0..v.len() - 1 {
        let diff: i32 = v[i].parse::<i32>().expect("error parsing")
            - v[i + 1].parse::<i32>().expect("error parsing");
        // if the diff is < 1 or > 3 unsafe
        if diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
        // if the succesion changes direcction unsafe
        if diff.signum() != inc {
            return false;
        }
    }
    true
}

// BRUTEFORCE!!!!
fn part2(v: Vec<&str>) -> bool {
    for i in 0..v.len() {
        let mut v1 = v.clone();
        v1.remove(i);
        if part1(v1) {
            return true;
        }
    }
    false
}
// Fancy didn't work
fn part2b(v: Vec<&str>) -> bool {
    if v.is_empty() || v.len() <= 2 {
        return true;
    };
    if v[0].parse::<i32>().expect("error parsing") == v[1].parse::<i32>().expect("error parsing") {
        return false;
    }
    let inc = if v[0].parse::<i32>().expect("error parsing")
        > v[1].parse::<i32>().expect("error parsing")
    {
        1
    } else {
        -1
    }; // -1 dec 1 inc if 0 we return false
    for i in 0..v.len() - 1 {
        let diff: i32 = v[i].parse::<i32>().expect("error parsing")
            - v[i + 1].parse::<i32>().expect("error parsing");
        // if the diff is < 1 delete one of the duplicates
        if diff.abs() < 1 {
            let mut v1 = v.clone();
            v1.remove(i);
            return part1(v1);
        }
        // if the succesion changes direcction or > 3 try removing 1
        if diff.signum() != inc || diff.abs() > 3 {
            for j in 0..v.len() {
                let mut v1 = v.clone();
                v1.remove(j);
                if part1(v1) {
                    return true;
                }
            }
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut res = 0;
        let reports = vec![
            vec!["7", "6", "4", "2", "1"], // Safe
            vec!["1", "2", "7", "8", "9"], // Unsafe (2 -> 7 increase of 5)
            vec!["9", "7", "6", "2", "1"], // Unsafe (6 -> 2 decrease of 4)
            vec!["1", "3", "2", "4", "5"], // Unsafe (1->3 increasing, 3->2 decreasing)
            vec!["8", "6", "4", "4", "1"], // Unsafe (4 -> 4 neither increasing nor decreasing)
            vec!["1", "3", "6", "7", "9"], // Safe
        ];

        for report in reports {
            if part1(report) {
                res += 1;
            }
        }
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part2() {
        let mut res = 0;
        let reports = vec![
            vec!["7", "6", "4", "2", "1"], // Safe
            vec!["1", "2", "7", "8", "9"], // Unsafe (2 -> 7 increase of 5)
            vec!["9", "7", "6", "2", "1"], // Unsafe (6 -> 2 decrease of 4)
            vec!["1", "3", "2", "4", "5"], // Safe (1->3 increasing, 3->2 decreasing) removing the 3
            vec!["8", "6", "4", "4", "1"], // Safe (4 -> 4 neither increasing nor decreasing) removing the 3
            vec!["1", "3", "6", "7", "9"], // Safe
        ];

        for report in reports {
            if part2(report) {
                res += 1;
            }
        }
        assert_eq!(res, 4);
    }

    #[test]
    fn test_part2_long() {
        let mut res = 0;
        let reports = vec![
            vec!["7", "6", "4", "2", "1"],     // Safe without removing any level
            vec!["1", "2", "7", "8", "9"],     // Unsafe regardless of which level is removed
            vec!["9", "7", "6", "2", "1"],     // Unsafe regardless of which level is removed
            vec!["1", "3", "2", "4", "5"],     // Safe by removing the second level (3)
            vec!["8", "6", "4", "4", "1"],     // Safe by removing the third level (4)
            vec!["1", "3", "6", "7", "9"],     // Safe without removing any level
            vec!["10", "9", "8", "7", "6"], // Safe without removing any level (strictly decreasing)
            vec!["5", "6", "9", "8", "7"],  // Unsafe regardless of which level is removed
            vec!["2", "3", "4", "5"],       // Safe without removing any level (strictly increasing)
            vec!["1"],                      // Safe (single level is trivially safe)
            vec![],                         // Safe (empty report is trivially safe)
            vec!["1", "4", "1", "4", "1"],  // Unsafe regardless of which level is removed
            vec!["1", "3", "5", "4", "6"],  // Safe by removing the fourth level (4)
            vec!["4", "5", "6", "3", "2"],  // Unsafe
            vec!["5", "8", "4", "3", "2"],  // Safe
            vec!["2", "2", "2", "2", "2"],  // Unsafe
            vec!["10", "9", "10", "11", "12"], // Unsafe
            vec!["1", "4", "3", "2"],       // Safe
            vec!["4", "4", "7", "8", "7", "8", "9", "13"], // Unsafe
        ];

        for report in reports {
            if part2(report) {
                res += 1;
            }
        }
        assert_eq!(res, 12);
    }

    #[test]
    fn test_part2_edge_cases() {
        let mut res = 0;
        let reports = vec![
            vec![],                        // Safe (empty report)
            vec!["1"],                     // Safe (single level)
            vec!["1", "2"],                // Safe (strictly increasing)
            vec!["2", "1"],                // Safe (strictly decreasing)
            vec!["1", "4"],                // Safe (difference 3 is valid)
            vec!["1", "5"],                // Safe (difference 4 exceeds limit)
            vec!["1", "3", "7"],           // Safe by removing third level (7)
            vec!["7", "5", "3", "1"],      // Safe (strictly decreasing)
            vec!["7", "5", "3", "4", "1"], // Safe by removing fourth level (4)
        ];

        for report in reports {
            if part2(report) {
                res += 1;
            }
        }
        assert_eq!(res, 9);
    }

    #[test]
    fn test_part2_no_safe() {
        let mut res = 0;
        let reports = vec![
            vec!["1", "5", "10"],            // Unsafe regardless of removal
            vec!["10", "20", "15"],          // Unsafe regardless of removal
            vec!["5", "5", "5"],             // Unsafe (no valid differences)
            vec!["1", "1", "1", "1"],        // Unsafe (no valid differences)
            vec!["10", "7", "3", "0", "-5"], // Unsafe regardless of removal
        ];

        for report in reports {
            if part2(report) {
                res += 1;
            }
        }
        assert_eq!(res, 0);
    }
}
