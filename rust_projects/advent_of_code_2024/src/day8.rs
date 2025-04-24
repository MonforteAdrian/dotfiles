use std::collections::HashSet;

// Function to parse the input map into a vector of antennas
fn parse_map(input: &str) -> Vec<(usize, usize, char)> {
    let mut antennas = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((row, col, c));
            }
        }
    }
    antennas
}

// Function to find all antinodes within the map
fn find_antinodes(
    antennas: &[(usize, usize, char)],
    height: usize,
    width: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i != j && antennas[i].2 == antennas[j].2 {
                let (x1, y1, _) = antennas[i];
                let (x2, y2, _) = antennas[j];

                // Calculate the directional distances
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Calculate antinode positions (one in each direction)
                let antinode1_x = x1 as isize - dx;
                let antinode1_y = y1 as isize - dy;
                let antinode2_x = x2 as isize + dx;
                let antinode2_y = y2 as isize + dy;

                // Add valid antinodes to the set
                if (0..height as isize).contains(&antinode1_x)
                    && (0..width as isize).contains(&antinode1_y)
                {
                    antinodes.insert((antinode1_x as usize, antinode1_y as usize));
                }
                if (0..height as isize).contains(&antinode2_x)
                    && (0..width as isize).contains(&antinode2_y)
                {
                    antinodes.insert((antinode2_x as usize, antinode2_y as usize));
                }
            }
        }
    }

    antinodes
}

// Function to find all antinodes within the map
fn find_antinodes2(
    antennas: &[(usize, usize, char)],
    height: usize,
    width: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in 0..antennas.len() {
            if i != j && antennas[i].2 == antennas[j].2 {
                let (x1, y1, _) = antennas[i];
                let (x2, y2, _) = antennas[j];

                // Calculate the directional distances
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let mut antinode1_x = x1 as isize;
                let mut antinode1_y = y1 as isize;
                let mut antinode2_x = x2 as isize;
                let mut antinode2_y = y2 as isize;
                let mut in1 = true;
                let mut in2 = true;
                while in1 || in2 {
                    // Calculate antinode positions (one in each direction)
                    antinode1_x -= dx;
                    antinode1_y -= dy;
                    antinode2_x += dx;
                    antinode2_y += dy;

                    // Add valid antinodes to the set
                    if (0..width as isize).contains(&antinode1_x)
                        && (0..height as isize).contains(&antinode1_y)
                    {
                        antinodes.insert((antinode1_x as usize, antinode1_y as usize));
                    } else {
                        in1 = false;
                    }
                    if (0..width as isize).contains(&antinode2_x)
                        && (0..height as isize).contains(&antinode2_y)
                    {
                        antinodes.insert((antinode2_x as usize, antinode2_y as usize));
                    } else {
                        in2 = false;
                    }
                }
            }
        }
    }

    // Include antenna positions as antinodes
    for &(x, y, _) in antennas {
        antinodes.insert((x, y));
    }

    antinodes
}

fn main() {
    let lines = std::fs::read_to_string("data/day8.txt").unwrap();
    let res1 = part1(&lines);
    let res2 = part2(&lines);
    println!("{:?}", res1);
    println!("{:?}", res2);
}

fn part1(input: &str) -> i32 {
    let antennas = parse_map(input);
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let antinodes = find_antinodes(&antennas, height, width);

    antinodes.len() as i32
}
fn part2(input: &str) -> i32 {
    let antennas = parse_map(input);
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let antinodes = find_antinodes2(&antennas, height, width);

    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let report = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(report), 14);
    }
    #[test]
    fn test_part2() {
        let report = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part2(report), 34);
    }
}
