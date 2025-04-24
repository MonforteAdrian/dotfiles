use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = std::fs::read_to_string("data/day20.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn parse_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (i32, i32),
    cost: i32, // Cost from start to this node
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want the min-heap, so reverse the order
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn astar(
    start: (i32, i32),
    goal: (i32, i32),
    neighbors: impl Fn((i32, i32)) -> Vec<((i32, i32), i32)>,
    heuristic: impl Fn((i32, i32), (i32, i32)) -> i32,
) -> Option<Vec<(i32, i32)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();

    open_set.push(Node {
        position: start,
        cost: 0,
    });

    g_score.insert(start, 0);

    while let Some(Node { position, .. }) = open_set.pop() {
        if position == goal {
            let mut path = vec![position];
            while let Some(&prev) = came_from.get(&path[path.len() - 1]) {
                path.push(prev);
            }
            path.reverse();
            return Some(path);
        }

        for (neighbor, cost) in neighbors(position) {
            let tentative_g_score = g_score.get(&position).unwrap_or(&i32::MAX) + cost;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, position);
                g_score.insert(neighbor, tentative_g_score);
                let f_score = tentative_g_score + heuristic(neighbor, goal);
                open_set.push(Node {
                    position: neighbor,
                    cost: f_score,
                });
            }
        }
    }

    None // No path found
}

use std::collections::{HashSet, VecDeque};

fn can_cheat(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    path: &[(i32, i32)],
    max_cheat_duration: i32,
) -> Vec<i32> {
    let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // Up, Left, Down, Right
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut valid_cheats = vec![];

    // Initialize BFS
    queue.push_back((start_pos, 0)); // (current position, cheat cost)
    visited.insert((start_pos, 0));

    while let Some((pos, cheat_cost)) = queue.pop_front() {
        if cheat_cost > max_cheat_duration {
            continue; // Exceeded cheat limit
        }

        for (dx, dy) in &directions {
            let nx = pos.0 + dx;
            let ny = pos.1 + dy;

            if nx < 0 || ny < 0 || nx as usize >= grid.len() || ny as usize >= grid[0].len() {
                continue; // Out of bounds
            }

            let cell = grid[ny as usize][nx as usize];
            let new_cheat_cost = cheat_cost + if cell == '#' { 1 } else { 0 };

            if new_cheat_cost > max_cheat_duration || visited.contains(&((nx, ny), 0)) {
                continue;
            }

            visited.insert(((nx, ny), 0));

            if cell == '.' || cell == 'E' {
                // Check if the cheat path ends at a valid location
                let Some(start_idx) = path.iter().position(|&p| p == start_pos) else {
                    continue;
                };
                let Some(end_idx) = path.iter().position(|&p| p == (nx, ny)) else {
                    continue;
                };

                if end_idx > start_idx {
                    let time_saved = (end_idx as i32 - start_idx as i32) - new_cheat_cost;
                    if time_saved >= 100 {
                        valid_cheats.push(time_saved);
                    }
                }
            }

            // Add to BFS queue
            queue.push_back(((nx, ny), new_cheat_cost));
        }
    }

    valid_cheats
}

fn find_shortcuts(path: &[(i32, i32)], max_cheat_duration: i32) -> i32 {
    path.par_iter()
        .enumerate()
        .map(|(i, &start)| {
            let mut shortcuts = 0;
            for (j, end) in path.iter().enumerate().skip(i + 1) {
                // Compute Manhattan distance
                let manhattan_distance = (end.0 - start.0).abs() + (end.1 - start.1).abs();

                if manhattan_distance <= max_cheat_duration {
                    let original_length = (j - i) as i32;
                    let time_saved = original_length - manhattan_distance;

                    if time_saved >= 100 {
                        shortcuts += 1;
                    }
                }
            }
            shortcuts
        })
        .sum() // Combine results from all threads
}

fn part1(s: &str) -> i32 {
    let grid = parse_grid(s);
    print_grid(&grid);
    let start = find_position(&grid, 'S').unwrap();
    let goal = find_position(&grid, 'E').unwrap();

    let neighbors = |pos: (i32, i32)| -> Vec<((i32, i32), i32)> {
        let (x, y) = pos;
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // Up, Left, Down, Right
        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && ny >= 0
                    && (nx as usize) < grid.len()
                    && (ny as usize) < grid[0].len()
                    && (grid[ny as usize][nx as usize] == '.'
                        || grid[ny as usize][nx as usize] == 'S'
                        || grid[ny as usize][nx as usize] == 'E')
                // Ensure it's not a blocked cell
                {
                    Some(((nx, ny), 1)) // Cost of moving is 1
                } else {
                    None
                }
            })
            .collect()
    };

    let heuristic = |pos: (i32, i32), goal: (i32, i32)| -> i32 {
        let (x1, y1) = pos;
        let (x2, y2) = goal;
        (x1 - x2).abs() + (y1 - y2).abs() // Manhattan distance
    };

    let Some(path) = astar(start, goal, neighbors, heuristic) else {
        return 0;
    };
    let mut res = 0;
    for point in &path {
        // Search cheat
        res += can_cheat(&grid, *point, &path.clone(), 2).len();
    }
    res as i32
}

fn part2(s: &str) -> i32 {
    let grid = parse_grid(s);
    print_grid(&grid);
    let start = find_position(&grid, 'S').unwrap();
    let goal = find_position(&grid, 'E').unwrap();

    let neighbors = |pos: (i32, i32)| -> Vec<((i32, i32), i32)> {
        let (x, y) = pos;
        let directions = [(0, -1), (-1, 0), (0, 1), (1, 0)]; // Up, Left, Down, Right
        directions
            .iter()
            .filter_map(|(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && ny >= 0
                    && (nx as usize) < grid.len()
                    && (ny as usize) < grid[0].len()
                    && (grid[ny as usize][nx as usize] == '.'
                        || grid[ny as usize][nx as usize] == 'S'
                        || grid[ny as usize][nx as usize] == 'E')
                {
                    Some(((nx, ny), 1)) // Cost of moving is 1
                } else {
                    None
                }
            })
            .collect()
    };

    let heuristic = |pos: (i32, i32), goal: (i32, i32)| -> i32 {
        let (x1, y1) = pos;
        let (x2, y2) = goal;
        (x1 - x2).abs() + (y1 - y2).abs() // Manhattan distance
    };

    let Some(path) = astar(start, goal, neighbors, heuristic) else {
        return 0;
    };

    // Find all shortcuts
    find_shortcuts(&path, 20)
}

fn print_grid(grid: &[Vec<char>]) {
    for r in grid {
        for ch in r {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn find_position(grid: &[Vec<char>], target: char) -> Option<(i32, i32)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == target {
                return Some((x as i32, y as i32));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let report = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!(part2(report), 285);
    }
}
