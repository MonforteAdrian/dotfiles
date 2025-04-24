use regex::Regex;
use std::fs;

fn main() {
    let mut res1 = 0;
    let mut res2 = 0;
    let lines = fs::read_to_string("data/day4.txt").unwrap();
    res1 = part1(parse_input(&lines));
    res2 = part2(parse_input(&lines));

    println!("{:?}", res1);
    println!("{:?}", res2);
}
fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn part1(grid: Vec<Vec<char>>) -> i32 {
    let word = "XMAS";
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    // Define 8 directions: (row_step, col_step)
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut match_found = true;
                for (i, char) in word_chars.iter().enumerate().take(word.len()) {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;
                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        match_found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != *char {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [
        (1, 1),   // Down-right
        (-1, -1), // Up-left
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
    ];
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 'A' {
                let mut match_found = false;
                if !(r < 1 || c < 1 || r == rows - 1 || c == cols - 1) {
                    let mut checked = 'A';
                    for &(dr, dc) in &directions {
                        match_found = true;
                        let nr = r as isize + dr as isize;
                        let nc = c as isize + dc as isize;
                        if grid[nr as usize][nc as usize] == 'A'
                            || grid[nr as usize][nc as usize] == 'X'
                        {
                            match_found = false;
                            break;
                        } else if dr == 1 {
                            checked = grid[nr as usize][nc as usize];
                        } else if checked == grid[nr as usize][nc as usize] {
                            match_found = false;
                            break;
                        }
                    }
                }
                if match_found {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(parse_input(report)), 18);
    }

    #[test]
    fn test_part2() {
        let report = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(parse_input(report)), 9);
    }
}
