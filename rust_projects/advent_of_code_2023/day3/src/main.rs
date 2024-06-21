use std::{fs, task::Wake};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error");
    println!("{}", part1(&file));
    println!("{}", part2(&file));
}

pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    let matrix: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let (mut number, mut start, mut end) = (0, 0, 0);
    for (i, line) in matrix.iter().enumerate() {
        'line: for (j, char) in line.iter().enumerate() {
            match char.to_digit(10) {
                Some(val) => {
                    if number == 0 {
                        (number, start, end) = (val, j, j);
                    } else {
                        (number, end) = (number * 10 + val, j);
                    }
                    if j == line.len() - 1 {
                        // Possible problem that the last digit is the last porition on the line so we will not get here
                        if number > 0 {
                            if start > 0 {
                                if line[start - 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                                if i > 0 {
                                    if matrix[i - 1][start - 1] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue;
                                    }
                                }
                                if i < matrix.len() - 1 {
                                    if matrix[i + 1][start - 1] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue;
                                    }
                                }
                            }
                            if end < line.len() - 1 {
                                if line[end + 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                                if i > 0 {
                                    if matrix[i - 1][end + 1] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue;
                                    }
                                }
                                if i < matrix.len() - 1 {
                                    if matrix[i + 1][end + 1] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue;
                                    }
                                }
                            }
                            if i > 0 {
                                for k in start..=end {
                                    if matrix[i - 1][k] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue 'line;
                                    }
                                }
                            }
                            if i < matrix.len() - 1 {
                                for k in start..=end {
                                    if matrix[i + 1][k] != '.' {
                                        res += number;
                                        (number, start, end) = (0, 0, 0);
                                        continue 'line;
                                    }
                                }
                            }
                            (number, start, end) = (0, 0, 0);
                        }
                    }
                }
                _ => {
                    // Possible problem that the last digit is the last porition on the line so we will not get here
                    if number > 0 {
                        if start > 0 {
                            if line[start - 1] != '.' {
                                res += number;
                                (number, start, end) = (0, 0, 0);
                                continue;
                            }
                            if i > 0 {
                                if matrix[i - 1][start - 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                            }
                            if i < matrix.len() - 1 {
                                if matrix[i + 1][start - 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                            }
                        }
                        if end < line.len() - 1 {
                            if line[end + 1] != '.' {
                                res += number;
                                (number, start, end) = (0, 0, 0);
                                continue;
                            }
                            if i > 0 {
                                if matrix[i - 1][end + 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                            }
                            if i < matrix.len() - 1 {
                                if matrix[i + 1][end + 1] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue;
                                }
                            }
                        }
                        if i > 0 {
                            for k in start..=end {
                                if matrix[i - 1][k] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue 'line;
                                }
                            }
                        }
                        if i < matrix.len() - 1 {
                            for k in start..=end {
                                if matrix[i + 1][k] != '.' {
                                    res += number;
                                    (number, start, end) = (0, 0, 0);
                                    continue 'line;
                                }
                            }
                        }
                        (number, start, end) = (0, 0, 0);
                    }
                }
            }
        }
    }
    res
}

pub fn part2(input: &str) -> u32 {
    let mut res = 0;
    let matrix: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let mut number = 1;
    let mut finds = 0;
    for (i, line) in matrix.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == '*' {
                if i > 0 {
                    if matrix[i - 1][j].is_digit(10) {
                        finds += 1;
                        number *= get_number(matrix[i - 1].clone(), j);
                    } else {
                        if j > 0 {
                            if matrix[i - 1][j - 1].is_digit(10) {
                                finds += 1;
                                number *= get_number(matrix[i - 1].clone(), j - 1);
                            }
                        }
                        if j < line.len() - 1 {
                            if matrix[i - 1][j + 1].is_digit(10) {
                                finds += 1;
                                number *= get_number(matrix[i - 1].clone(), j + 1);
                            }
                        }
                    }
                }
                if i < matrix.len() - 1 {
                    if matrix[i + 1][j].is_digit(10) {
                        finds += 1;
                        number *= get_number(matrix[i + 1].clone(), j);
                    } else {
                        if j > 0 {
                            if matrix[i + 1][j - 1].is_digit(10) {
                                finds += 1;
                                number *= get_number(matrix[i + 1].clone(), j - 1);
                            }
                        }
                        if j < line.len() - 1 {
                            if matrix[i + 1][j + 1].is_digit(10) {
                                finds += 1;
                                number *= get_number(matrix[i + 1].clone(), j + 1);
                            }
                        }
                    }
                }
                if j > 0 {
                    if line[j - 1].is_digit(10) {
                        finds += 1;
                        number *= get_number(line.clone(), j - 1);
                    }
                }
                if j < line.len() - 1 {
                    if line[j + 1].is_digit(10) {
                        finds += 1;
                        number *= get_number(line.clone(), j + 1);
                    }
                }
                if finds == 2 {
                    println!("{number}");
                    res += number;
                }
                number = 1;
                finds = 0;
            }
        }
    }
    res
}

fn get_number(input: Vec<char>, index: usize) -> u32 {
    for i in (0..=index).rev() {
        if !input[i].is_digit(10) {
            let mut res = 0;
            for j in (i + 1)..input.len() {
                if !input[j].is_digit(10) {
                    return res;
                }
                if j == input.len() - 1 {
                    res = res * 10 + input[j].to_digit(10).unwrap();
                    return res;
                }
                res = res * 10 + input[j].to_digit(10).unwrap();
            }
        } else if i == 0 {
            let mut res = 0;
            for j in i..input.len() {
                if !input[j].is_digit(10) {
                    return res;
                }
                if j == input.len() - 1 {
                    res = res * 10 + input[j].to_digit(10).unwrap();
                    return res;
                }
                res = res * 10 + input[j].to_digit(10).unwrap();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_test() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
15.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";
        let result = part2(input);
        assert_eq!(result, 6756);
    }
}
