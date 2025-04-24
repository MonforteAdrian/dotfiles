fn main() {
    let lines = std::fs::read_to_string("data/day6.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(s: &str) -> i32 {
    let mut grid = parse_input(s);
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    // Find starting position and direction
    let (mut r, mut c) = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|&ch| ch == '^')
                .map(|col_idx| (row_idx as i32, col_idx as i32))
        })
        .unwrap_or((0, 0)); // Default to (0, 0) if not found
    let mut d = 0; // Initially facing up

    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    // Use a set to track visited positions
    let mut visited = std::collections::HashSet::new();
    visited.insert((r, c));

    loop {
        let new_r = r + directions[d].0;
        let new_c = c + directions[d].1;

        // Check if out of bounds
        if new_r < 0 || new_r >= height || new_c < 0 || new_c >= width {
            break;
        }

        match grid[new_r as usize][new_c as usize] {
            '#' => {
                // Turn right (90 degrees clockwise)
                d = (d + 1) % 4;
            }
            '.' => {
                // Move forward
                r = new_r;
                c = new_c;

                // Mark the position as visited
                if visited.insert((r, c)) {
                    // Mark the grid to avoid revisiting
                    grid[r as usize][c as usize] = 'X';
                }
            }
            _ => {
                // If guard reaches 'X' or any other character, continue moving forward
                r = new_r;
                c = new_c;
            }
        }
    }

    visited.len() as i32
}

fn part2(s: &str) -> i32 {
    let grid = parse_input(s);
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let directions = [
        (-1, 0), // Up
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
    ];

    // Find starting position and direction
    let (start_r, start_c) = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, row)| {
            row.iter()
                .position(|&ch| ch == '^')
                .map(|col_idx| (row_idx as i32, col_idx as i32))
        })
        .unwrap();

    let mut loop_positions = std::collections::HashSet::new();

    // Test every open position on the grid
    for r in 0..height {
        for c in 0..width {
            // Skip if the position is already an obstruction or the starting position
            if grid[r as usize][c as usize] == '#' || (r == start_r && c == start_c) {
                continue;
            }

            // Create a temporary grid with a new obstruction at (r, c)
            let mut temp_grid = grid.clone();
            temp_grid[r as usize][c as usize] = '#';

            // Simulate guard movement
            if causes_loop(&temp_grid, start_r, start_c, &directions) {
                loop_positions.insert((r, c));
            }
        }
    }

    loop_positions.len() as i32
}

fn causes_loop(
    grid: &Vec<Vec<char>>,
    start_r: i32,
    start_c: i32,
    directions: &[(i32, i32)],
) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut visited_states = std::collections::HashSet::new();
    let mut r = start_r;
    let mut c = start_c;
    let mut d = 0; // Initially facing up

    while visited_states.insert((r, c, d)) {
        let new_r = r + directions[d].0;
        let new_c = c + directions[d].1;

        // Check if out of bounds
        if new_r < 0 || new_r >= height || new_c < 0 || new_c >= width {
            return false; // Guard exits the grid
        }

        match grid[new_r as usize][new_c as usize] {
            '#' => {
                // Turn right (90 degrees clockwise)
                d = (d + 1) % 4;
            }
            '.' => {
                // Move forward
                r = new_r;
                c = new_c;
            }
            _ => {
                // For any other character, move forward
                r = new_r;
                c = new_c;
            }
        }
    }

    true // Loop detected
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(report), 41);
    }
    #[test]
    fn test_part2() {
        let report = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part2(report), 6);
    }
}
