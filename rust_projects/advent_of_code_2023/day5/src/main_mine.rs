use rayon::prelude::*;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let seeds_line: Vec<_> = lines.first().expect("Error").split(":").collect();
    let mut seeds: Vec<u32> = seeds_line
        .last()
        .expect("Error")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut aux = seeds.clone();
    for line in lines {
        if line.is_empty() {
            aux = seeds.clone();
            continue;
        }
        if line.chars().nth(0).expect("Error").is_digit(10) {
            let values: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            for (i, seed) in aux.iter().enumerate() {
                if seed >= &values[1] {
                    if seed - values[1] < values[2] {
                        if values[0] > values[1] {
                            seeds[i] += values[0] - values[1];
                        } else {
                            seeds[i] -= values[1] - values[0];
                        }
                    }
                }
            }
        }
    }
    *seeds.iter().min().expect("Error")
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let seeds_line: Vec<_> = lines.first().expect("Error").split(":").collect();
    let seeds_and_ranges: Vec<u32> = seeds_line
        .last()
        .expect("Error")
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut seeds: Vec<_> = vec![];
    for (i, seed_and_range) in seeds_and_ranges.iter().enumerate() {
        if i % 2 == 0 {
            let next = seeds_and_ranges[i + 1];
            let v: Vec<_> = vec![];
            (*seed_and_range..seed_and_range + next)
                .into_par_iter()
                .for_each(|x| v.push(x));
            seeds.push(v);
        }
    }
    let mut aux = seeds.clone();
    for line in lines {
        if line.is_empty() {
            aux = seeds.clone();
            continue;
        }
        if line.chars().nth(0).expect("Error").is_digit(10) {
            let values: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            for (i, range) in aux.iter().enumerate() {
                for (j, seed) in range.iter().enumerate() {
                    if seed >= &values[1] {
                        if seed - values[1] < values[2] {
                            if values[0] > values[1] {
                                seeds[i][j] += values[0] - values[1];
                            } else {
                                seeds[i][j] -= values[1] - values[0];
                            }
                        }
                    }
                }
            }
        }
    }
    *seeds
        .iter()
        .map(|v| v.iter().min().expect(""))
        .min()
        .expect("Error")
}

//Tuples try the problem is that the seeds try to change rages already changed
//pub fn part2(input: &str) -> u32 {
//    let lines: Vec<_> = input.lines().collect();
//    let seeds_line: Vec<_> = lines.first().expect("Error").split(":").collect();
//    let seeds_and_ranges: Vec<u32> = seeds_line
//        .last()
//        .expect("Error")
//        .trim()
//        .split_whitespace()
//        .map(|x| x.parse::<u32>().unwrap())
//        .collect();
//    let mut seeds: Vec<_> = vec![];
//    for (i, seed_and_range) in seeds_and_ranges.iter().enumerate() {
//        if i % 2 == 0 {
//            seeds.push((seed_and_range.clone(), seeds_and_ranges[i + 1]));
//        }
//    }
//    println!("{:?}", seeds);
//    let mut aux = seeds.clone();
//    for line in lines {
//        if line.is_empty() {
//            aux = seeds.clone();
//            continue;
//        }
//        if line.chars().nth(0).expect("Error").is_digit(10) {
//            let values: Vec<u32> = line
//                .split_whitespace()
//                .map(|x| x.parse::<u32>().unwrap())
//                .collect();
//            for (i, (start, range)) in aux.iter().enumerate() {
//                let end = *start + *range;
//                let last = &(values[1] + values[2]);
//                if start > last || end < values[1] {
//                    continue;
//                } else if start < &values[1] && end < *last {
//                    seeds[i].1 = range - (end - &values[1]);
//                    seeds.push((values[1], end - &values[1]));
//                } else if start >= &values[1] && end <= *last {
//                    if values[0] > values[1] {
//                        seeds[i].0 = start + (values[0] - values[1]);
//                    } else {
//                        seeds[i].0 = start - (values[1] - values[0]);
//                    }
//                } else if start > &values[1] && end > *last {
//                    if values[0] > values[1] {
//                        seeds[i] = (start + (values[0] - values[1]), last - start);
//                    } else {
//                        seeds[i] = (start - (values[1] - values[0]), last - start);
//                    }
//                    seeds.push((*last, end - last));
//                } else {
//                    seeds[i].1 = values[1] - start;
//                    seeds.push((values[0], values[2]));
//                    seeds.push((*last, end - last));
//                }
//            }
//        }
//    }
//    *seeds.iter().map(|(x, _)| x).min().expect("Error")
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part2(input);
        assert_eq!(result, 46);
    }
}
