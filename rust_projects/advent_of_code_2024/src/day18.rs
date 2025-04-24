use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
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

fn part1(s: &str) -> usize {
    //70 x 70 || 6  x 6(example)
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    // take only the first 12 bytes or 1024 for the full example
    for line in s.lines().take(1024) {
        let Some((x, y)) = line.split(",").collect_tuple() else {
            todo!()
        };
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';
    }
    print_grid(&grid);
    let start = (0, 0);
    let goal = (70, 70);

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
                    && grid[ny as usize][nx as usize] == '.'
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

    if let Some(path) = astar(start, goal, neighbors, heuristic) {
        for point in &path {
            grid[point.1 as usize][point.0 as usize] = 'O';
        }
        print_grid(&grid);
        path.len() - 1
    } else {
        0
    }
}
fn part2(s: &str) -> String {
    //70 x 70 || 6  x 6(example)
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    let mut res = (0, 0);
    // take only the first 12 bytes
    for line in s.lines() {
        let Some((x, y)) = line.split(",").collect_tuple() else {
            todo!()
        };
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = '#';
        print_grid(&grid);
        let start = (0, 0);
        let goal = (70, 70);

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
                        && grid[ny as usize][nx as usize] == '.'
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

        if astar(start, goal, neighbors, heuristic).is_none() {
            res = (x.parse().unwrap(), y.parse().unwrap());
            break;
        };
    }
    format!("{},{}", res.0, res.1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(part1(report), 22);
    }
}
