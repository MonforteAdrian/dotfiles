use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{thread, time};

fn main() {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    let res1 = part1(&input);
    println!("Total Cost: {:?}", res1);
}

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    direction: usize,
    steps: usize,    // Consecutive forward steps
    path: Vec<char>, // Path taken to reach this state
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let (start_x, start_y) = find_position(&grid, 'S').unwrap();
    let (end_x, end_y) = find_position(&grid, 'E').unwrap();

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // E, S, W, N
    let direction_chars = ['>', 'v', '<', '^'];

    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![vec![false; 4]; grid[0].len()]; grid.len()];

    heap.push(Reverse(State {
        cost: 0,
        x: start_x,
        y: start_y,
        direction: 0,
        steps: 0,
        path: vec!['S'], // Starting point
    }));

    while let Some(Reverse(state)) = heap.pop() {
        let State {
            cost,
            x,
            y,
            direction,
            steps,
            path,
        } = state;

        if x == end_x && y == end_y {
            println!("Reached the end with cost: {}", cost);
            println!("Path: {}", path.iter().collect::<String>());
            return cost;
        }

        if visited[y][x][direction] {
            continue;
        }
        visited[y][x][direction] = true;

        // Debug Visualization
        print_grid_with_position(&grid, x, y, direction_chars[direction]);
        println!(
            "Cost: {}, Steps: {}, Direction: {}",
            cost, steps, direction_chars[direction]
        );
        thread::sleep(time::Duration::from_millis(100)); // Slow down for clarity

        // Move forward first
        let (dx, dy) = directions[direction];
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && ny < grid.len() as isize && nx < grid[0].len() as isize {
            let nx = nx as usize;
            let ny = ny as usize;

            if grid[ny][nx] != '#' {
                let mut new_path = path.clone();
                new_path.push('F'); // Forward move
                heap.push(Reverse(State {
                    cost: cost + 1,
                    x: nx,
                    y: ny,
                    direction,
                    steps: steps + 1,
                    path: new_path,
                }));
            }
        }

        // Turn clockwise
        let cw_direction = (direction + 1) % 4;
        let mut cw_path = path.clone();
        cw_path.push('R'); // Turn Right
        heap.push(Reverse(State {
            cost: cost + 1000,
            x,
            y,
            direction: cw_direction,
            steps: 0,
            path: cw_path,
        }));

        // Turn counterclockwise
        let ccw_direction = (direction + 3) % 4;
        let mut ccw_path = path.clone();
        ccw_path.push('L'); // Turn Left
        heap.push(Reverse(State {
            cost: cost + 1000,
            x,
            y,
            direction: ccw_direction,
            steps: 0,
            path: ccw_path,
        }));
    }

    0
}

fn find_position(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == target {
                return Some((x, y));
            }
        }
    }
    None
}

fn print_grid_with_position(grid: &[Vec<char>], x: usize, y: usize, direction_char: char) {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if row_idx == y && col_idx == x {
                print!("{}", direction_char); // Mark the reindeer's position with its direction
            } else {
                print!("{}", ch);
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part1(report), 7036);
    }
}
