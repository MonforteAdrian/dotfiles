fn main() {
    let lines = std::fs::read_to_string("data/day9.txt").unwrap();
    // just run it if needed will take several minutes
    //let res1 = part1(&lines);
    let res2 = part2(&lines);
    //println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(input: &str) -> i64 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut v = vec![];
    let mut id: u32 = 0;
    // create the individual blocks
    s.iter().enumerate().for_each(|(i, digit)| {
        let value: String;
        // if file block else free space
        if i % 2 == 0 {
            value = id.to_string();
            id += 1;
        } else {
            value = ".".to_string();
        }
        for _ in 0..*digit as i32 {
            v.push(value.clone());
        }
    });
    // put free disk at the end
    for i in 0..v.len() {
        if v[i] == "." {
            for j in (0..v.len()).rev() {
                if v[j] != "." && i < j {
                    v.swap(i, j);
                    break;
                }
            }
        }
    }
    // do the math
    let res: i64 = v
        .iter()
        .filter(|x| *x != ".")
        .enumerate()
        .map(|(i, digit)| i as i64 * digit.parse::<i64>().unwrap())
        .sum();

    res
}
fn part2(input: &str) -> i64 {
    let s: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut v = vec![];
    let mut id: u32 = 0;
    // create the individual blocks
    s.iter().enumerate().for_each(|(i, digit)| {
        let value: String;
        // if file block else free space
        if i % 2 == 0 {
            value = id.to_string();
            id += 1;
        } else {
            value = ".".to_string();
        }
        for _ in 0..*digit as i32 {
            v.push(value.clone());
        }
    });
    // fragmennt the disk
    let mut i = (v.len() - 1) as i32;
    while i > 0 {
        if v[i as usize] != "." {
            let mut count = 0;
            for k in (0..=i).rev() {
                if v[k as usize] == v[i as usize] {
                    count += 1;
                } else {
                    break;
                }
            }
            let mut found = false;
            let mut j = 0;
            while j < v.len() {
                if v[j] == "." && i as usize > j {
                    let mut count2 = 0;
                    for k in j..v.len() {
                        if v[k] == v[j] {
                            count2 += 1;
                        } else {
                            break;
                        }
                    }
                    if count > count2 {
                        j += count2;
                    } else {
                        found = true;
                        for _ in 0..count {
                            v.swap(i as usize, j);
                            j += 1;
                            i -= 1;
                        }
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            if !found {
                i -= count as i32;
            }
        } else {
            i -= 1;
        }
    }
    // do the math
    let res: i64 = v
        .iter()
        .enumerate()
        .map(|(i, digit)| {
            if digit != "." {
                i as i64 * digit.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .sum();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_easy() {
        let report = "12345";
        assert_eq!(part1(report), 60);
    }

    #[test]
    fn test_part1_example() {
        let report = "2333133121414131402";
        assert_eq!(part1(report), 1928);
    }

    #[test]
    fn test_part2_example() {
        let report = "2333133121414131402";
        assert_eq!(part2(report), 2858);
    }
}
