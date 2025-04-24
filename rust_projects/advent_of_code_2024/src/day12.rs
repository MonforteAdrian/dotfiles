use std::collections::HashSet;

fn main() {
    let lines = std::fs::read_to_string("data/day12.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn flood_fill(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    x: isize,
    y: isize,
    plant_type: char,
) -> (i32, i32) {
    let mut stack = vec![(x, y)];
    let mut area = 0;
    let mut perimeter = 0;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((cx, cy)) = stack.pop() {
        if visited[cx as usize][cy as usize] {
            continue;
        }
        visited[cx as usize][cy as usize] = true;
        area += 1;

        for (dx, dy) in &directions {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
                perimeter += 1;
            } else {
                let (nx, ny) = (nx as isize, ny as isize);
                if grid[nx as usize][ny as usize] != plant_type {
                    perimeter += 1;
                } else if !visited[nx as usize][ny as usize] {
                    stack.push((nx, ny));
                }
            }
        }
    }

    (area, perimeter)
}

fn calculate_region(
    grid: &[Vec<char>],
    visited: &mut [Vec<bool>],
    start_x: isize,
    start_y: isize,
    plant_type: char,
) -> (i32, i32) {
    let mut stack = vec![(start_x, start_y)];
    let mut region_points = HashSet::new();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((x, y)) = stack.pop() {
        if visited[x as usize][y as usize] {
            continue;
        }
        visited[x as usize][y as usize] = true;
        region_points.insert((x, y));

        for (dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && ny >= 0 && nx < grid.len() as isize && ny < grid[0].len() as isize {
                let (nx, ny) = (nx, ny);
                if grid[nx as usize][ny as usize] == plant_type
                    && !visited[nx as usize][ny as usize]
                {
                    stack.push((nx, ny));
                }
            }
        }
    }

    let mut edges: Vec<(isize, isize, &str)> = vec![];

    for &(x, y) in &region_points {
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || ny < 0 || nx >= grid.len() as isize || ny >= grid[0].len() as isize {
                let dir = (nx - x, ny - y);
                let side = match dir {
                    (0, ..0) => "Left",
                    (..0, 0) => "Top",
                    (0, 0..) => "Right",
                    (0.., 0) => "Bottom",
                    _ => "Nope",
                };
                edges.push((x, y, side));
            } else {
                let (nx, ny) = (nx, ny);
                if !region_points.contains(&(nx, ny)) {
                    let dir = (nx - x, ny - y);
                    let side = match dir {
                        (0, ..0) => "Left",
                        (..0, 0) => "Top",
                        (0, 0..) => "Right",
                        (0.., 0) => "Bottom",
                        _ => "Nope",
                    };
                    edges.push((x, y, side));
                }
            }
        }
    }

    let mut segments = 0;
    for side_pos in ["Top", "Bottom"] {
        let mut side_edges: Vec<(isize, isize)> = edges
            .iter()
            .filter(|&&(_, _, side)| side == side_pos)
            .map(|&(x, y, _)| (x, y))
            .collect();
        side_edges.sort();

        let mut prev_x = None;
        let mut prev_y = None;

        for &(x, y) in &side_edges {
            if prev_x == Some(x) && prev_y.map_or(false, |py| py + 1 == y) {
                prev_y = Some(y);
            } else {
                segments += 1;
                prev_x = Some(x);
                prev_y = Some(y);
            }
        }
    }
    for side_pos in ["Right", "Left"] {
        let mut side_edges: Vec<(isize, isize)> = edges
            .iter()
            .filter(|&&(_, _, side)| side == side_pos)
            .map(|&(x, y, _)| (x, y))
            .collect();
        side_edges.sort_by(|&(x1, y1), &(x2, y2)| y1.cmp(&y2).then(x1.cmp(&x2)));

        let mut prev_x = None;
        let mut prev_y = None;

        for &(x, y) in &side_edges {
            if prev_y == Some(y) && prev_x.map_or(false, |px| px + 1 == x) {
                prev_x = Some(x);
            } else {
                segments += 1;
                prev_x = Some(x);
                prev_y = Some(y);
            }
        }
    }

    (region_points.len() as i32, segments)
}

fn part1(s: &str) -> i32 {
    let grid = parse_input(s);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total_price = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let (area, perimeter) =
                    flood_fill(&grid, &mut visited, i as isize, j as isize, grid[i][j]);
                total_price += area * perimeter;
            }
        }
    }
    total_price
}

fn part2(s: &str) -> i32 {
    let grid = parse_input(s);
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut total_price = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited[i][j] {
                let (area, sides) =
                    calculate_region(&grid, &mut visited, i as isize, j as isize, grid[i][j]);
                total_price += area * sides;
            }
        }
    }
    total_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_easy() {
        let report = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part1(report), 140);
    }

    #[test]
    fn test_part1_medium() {
        let report = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part1(report), 772);
    }

    #[test]
    fn test_part1_hard() {
        let report = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part1(report), 1930);
    }

    #[test]
    fn test_part2_easy() {
        let report = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(part2(report), 80);
    }

    #[test]
    fn test_part2_medium() {
        let report = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(part2(report), 236);
    }

    #[test]
    fn test_part2_medium_2() {
        let report = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(part2(report), 436);
    }

    #[test]
    fn test_part2_hard() {
        let report = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(part2(report), 368);
    }

    #[test]
    fn test_part2_hard_2() {
        let report = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part2(report), 1206);
    }
}
