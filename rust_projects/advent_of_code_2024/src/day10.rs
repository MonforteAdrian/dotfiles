use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let lines = std::fs::read_to_string("data/day10.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_trailheads(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((i, j));
            }
        }
    }
    trailheads
}

fn bfs_trail(grid: &[Vec<u8>], start: (usize, usize)) -> HashSet<(usize, usize)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        let current_height = grid[x][y];
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && (nx as usize) < grid.len() && (ny as usize) < grid[0].len() {
                let (nx, ny) = (nx as usize, ny as usize);
                let next_height = grid[nx][ny];
                if !visited.contains(&(nx, ny)) && next_height == current_height + 1 {
                    visited.insert((nx, ny));
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    visited
        .into_iter()
        .filter(|&(x, y)| grid[x][y] == 9)
        .collect()
}

fn calculate_scores(grid: &[Vec<u8>], trailheads: Vec<(usize, usize)>) -> usize {
    trailheads
        .iter()
        .map(|&start| bfs_trail(grid, start).len())
        .sum()
}

fn dfs_trails(
    grid: &[Vec<u8>],
    x: usize,
    y: usize,
    current_height: u8,
    memo: &mut HashMap<(usize, usize, u8), usize>,
) -> usize {
    // Check memoized result
    if let Some(&result) = memo.get(&(x, y, current_height)) {
        return result;
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut total_trails = 0;

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < grid.len() && (ny as usize) < grid[0].len() {
            let (nx, ny) = (nx as usize, ny as usize);
            let next_height = grid[nx][ny];
            if next_height == current_height + 1 {
                total_trails += dfs_trails(grid, nx, ny, next_height, memo);
            }
        }
    }

    // Base case: If no valid moves and current height is 9, this is a valid trail
    if current_height == 9 {
        total_trails += 1;
    }

    // Memoize the result
    memo.insert((x, y, current_height), total_trails);
    total_trails
}

fn calculate_ratings(grid: &[Vec<u8>], trailheads: Vec<(usize, usize)>) -> usize {
    let mut memo = HashMap::new();
    trailheads
        .iter()
        .map(|&(x, y)| dfs_trails(grid, x, y, 0, &mut memo))
        .sum()
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    dbg!(grid.len());
    let trailheads = find_trailheads(&grid);
    //dbg!(&trailheads);
    calculate_scores(&grid, trailheads)
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let trailheads = find_trailheads(&grid);
    calculate_ratings(&grid, trailheads)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "0123
1234
8765
9876";
        assert_eq!(part1(report), 1);
    }

    #[test]
    fn test_part1_example() {
        let report = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(report), 36);
    }
}
