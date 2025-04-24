use std::fs;

fn main() {
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let values_str: Vec<&str> = line.trim().split("   ").collect();
        for (i, value) in values_str.iter().enumerate() {
            if i % 2 == 0 {
                arr1.push(value.parse().expect("error parsing"));
            } else {
                arr2.push(value.parse().expect("error parsing"));
            }
        }
    }

    println!("{:?}", part1(arr1.clone(), arr2.clone()));
    println!("{:?}", part2(arr1, arr2));
}

fn part1(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
    arr1.sort();
    arr2.sort();
    let mut res: i32 = 0;
    for i in 0..arr1.len() {
        res += (arr1[i] - arr2[i]).abs();
    }
    res
}

fn part2(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    for elem1 in arr1 {
        let mut count: i32 = 0;
        for elem2 in arr2.iter() {
            if elem1 == *elem2 {
                count += 1;
            }
        }
        res += elem1 * count;
    }
    res
}
