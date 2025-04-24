use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("data/day15.txt").unwrap();
    let res1 = part1(&input);
    let res2 = part2(&input);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn parse_input(s: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let parts: Vec<&str> = s.split("\n\n").collect();
    let grid = parts[0]
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let moves = parts[1].trim().chars().collect();
    (grid, moves)
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

fn print_moves(moves: &Vec<char>) {
    for ch in moves {
        print!("{}", ch);
    }
    println!();
}

fn move_robot(grid: &mut [Vec<char>], dir: (i32, i32)) {
    let Some(robot) = grid.iter().flatten().position(|&x| x == '@') else {
        return;
    };
    let (robot_x, robot_y) = (robot % grid.len(), robot / grid[0].len());
    let mut nx = robot_x as i32 + dir.0;
    let mut ny = robot_y as i32 + dir.1;
    if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
        return;
    }
    match grid[ny as usize][nx as usize] {
        '.' => {
            grid[ny as usize][nx as usize] = '@';
            grid[robot_y][robot_x] = '.'
        }
        'O' => {
            let mut count = 0;
            loop {
                nx += dir.0;
                ny += dir.1;
                if grid[ny as usize][nx as usize] == '.' {
                    break;
                }
                if grid[ny as usize][nx as usize] == '#' {
                    return;
                }
                count += 1;
            }
            while count >= 0 {
                let px = nx - dir.0;
                let py = ny - dir.1;
                grid[ny as usize][nx as usize] = 'O';
                nx = px;
                ny = py;
                count -= 1;
            }
            grid[ny as usize][nx as usize] = '@';
            grid[robot_y][robot_x] = '.'
        }
        _ => {}
    }
}

fn part1(s: &str) -> usize {
    let (mut grid, moves) = parse_input(s);
    //print_moves(&moves);
    let (top, right, down, left) = ((0, -1), (1, 0), (0, 1), (-1, 0));

    for mov in &moves {
        //print_grid(&grid);
        match mov {
            '^' => {
                move_robot(&mut grid, top);
            }
            '>' => {
                move_robot(&mut grid, right);
            }
            'v' => {
                move_robot(&mut grid, down);
            }
            '<' => {
                move_robot(&mut grid, left);
            }
            _ => {}
        }
    }
    let mut res = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == 'O' {
                res += row_idx * 100 + col_idx;
            }
        }
    }
    res
}

fn part2(s: &str) -> usize {
    let (grid, moves) = parse_input(s);
    let mut grid = expand_grid(&grid); // Expand the grid horizontally

    let directions = [((0, -1), '^'), ((1, 0), '>'), ((0, 1), 'v'), ((-1, 0), '<')];

    print_grid(&grid);
    for mov in moves {
        let dir = match mov {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => (0, 0),
        };
        move_robot_2(&mut grid, dir);
        print_grid(&grid);
    }

    calculate_gps(&grid)
}

fn expand_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_grid = Vec::new();
    for row in grid {
        let mut new_row = Vec::new();
        for &cell in row {
            match cell {
                '#' => new_row.extend(vec!['#', '#']), // Double walls
                'O' => new_row.extend(vec!['[', ']']), // Wide boxes
                '.' => new_row.extend(vec!['.', '.']), // Empty spaces
                '@' => new_row.extend(vec!['@', '.']), // Robot and expanded space
                _ => {}
            }
        }
        new_grid.push(new_row);
    }
    new_grid
}

fn move_robot_2(grid: &mut [Vec<char>], dir: (i32, i32)) {
    let Some(robot_pos) = grid.iter().flatten().position(|&x| x == '@') else {
        return;
    };

    let (robot_x, robot_y) = (robot_pos % grid[0].len(), robot_pos / grid[0].len());
    let (mut nx, mut ny) = (robot_x as i32 + dir.0, robot_y as i32 + dir.1);

    if nx < 0 || ny < 0 || ny >= grid.len() as i32 || nx >= grid[0].len() as i32 {
        return; // Out of bounds
    }

    let target = grid[ny as usize][nx as usize];

    match target {
        '.' => {
            // Move robot into an empty space
            grid[ny as usize][nx as usize] = '@';
            grid[robot_y][robot_x] = '.';
        }
        '[' | ']' => {
            let mut boxes = HashSet::new();
            let mut cx = nx;
            let mut cy = ny;

            if dir.0 == 0 {
                // Collect consecutive boxes
                let value = grid[cy as usize][cx as usize];
                if value == '[' {
                    boxes.insert((cx + 1, cy, grid[cy as usize][(cx + 1) as usize]));
                } else {
                    boxes.insert((cx - 1, cy, grid[cy as usize][(cx - 1) as usize]));
                }
                boxes.insert((cx, cy, grid[cy as usize][cx as usize]));
                let mut visited = Vec::new();
                let mut finished = false;
                while !finished {
                    finished = true;
                    for (x, y, _) in &boxes.clone() {
                        // add pair of the detected box
                        cx = x + dir.0;
                        cy = y + dir.1;
                        if (grid[cy as usize][cx as usize] == '['
                            || grid[cy as usize][cx as usize] == ']')
                            && !visited.contains(&(cx - dir.0, cy - dir.1))
                        {
                            finished = false;
                            boxes.insert((cx, cy, grid[cy as usize][cx as usize]));
                            visited.push((*x, *y));
                            if grid[cy as usize][cx as usize] == '[' {
                                boxes.insert((cx + 1, cy, grid[cy as usize][(cx + 1) as usize]));
                            } else {
                                boxes.insert((cx - 1, cy, grid[cy as usize][(cx - 1) as usize]));
                            }
                        }

                        if cx < 0 || cy < 0 || cy >= grid.len() as i32 || cx >= grid[0].len() as i32
                        {
                            return; // Out of bounds
                        }
                    }
                }
                // Ensure there is enough space to move all the boxes
                for &(bx, by, _) in &boxes {
                    let ahead_x = bx + dir.0;
                    let ahead_y = by + dir.1;
                    if grid[ahead_y as usize][ahead_x as usize] == '#' {
                        return; // Blocked, cannot move
                    }
                }
            } else {
                // Collect consecutive boxes
                while grid[cy as usize][cx as usize] == '[' || grid[cy as usize][cx as usize] == ']'
                {
                    boxes.insert((cx, cy, grid[cy as usize][cx as usize]));
                    cx += dir.0;
                    cy += dir.1;

                    if cx < 0 || cy < 0 || cy >= grid.len() as i32 || cx >= grid[0].len() as i32 {
                        return; // Out of bounds
                    }
                }
                // Ensure there is enough space to move all the boxes
                if grid[cy as usize][cx as usize] != '.' {
                    return; // Blocked, cannot move
                }
            }
            let mut boxes: Vec<(i32, i32, char)> = boxes.into_iter().collect();
            match dir {
                (0, -1) => {
                    boxes.sort_by(|a, b| a.1.cmp(&b.1));
                }
                (0, 1) => {
                    boxes.sort_by(|a, b| b.1.cmp(&a.1));
                }
                (1, 0) => {
                    boxes.sort_by(|a, b| b.0.cmp(&a.0));
                }
                (-1, 0) => {
                    boxes.sort_by(|a, b| a.0.cmp(&b.0));
                }
                _ => {}
            }

            // Move all boxes together
            for &(bx, by, value) in boxes.iter() {
                let ahead_x = bx + dir.0;
                let ahead_y = by + dir.1;
                grid[ahead_y as usize][ahead_x as usize] = value;
                grid[by as usize][bx as usize] = '.';
            }

            // Move the robot into the first box's original position
            grid[ny as usize][nx as usize] = '@';
            grid[robot_y][robot_x] = '.';
        }
        _ => {}
    }
}

fn calculate_gps(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '[' {
                // Left edge of a wide box
                sum += row_idx * 100 + col_idx;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(part1(report), 2028);
    }

    #[test]
    fn test_part1_large() {
        let report = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part1(report), 10092);
    }

    #[test]
    fn test_part2() {
        let report = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(part2(report), 9021);
    }
}
